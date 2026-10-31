//! A 3D model (STL, OBJ or 3MF) seen from above as a depth map: the highest point of the model is white (left uncut), the lowest is
//! black (the deepest), and where the model has nothing the map is black (the material round it is cleared). The map is a picture the
//! 3D relief of a layer is cut from.
use base_core::image::ImageData;

type Tri = [[f32; 3]; 3];

/// The longest side of a depth map in pixels.
const MAX_SIDE: f64 = 1600.0;
/// The most triangles read (a file with more is refused).
const MAX_TRIANGLES: usize = 5_000_000;

/// Is this a file the program reads as a 3D model?
pub fn is_model_file(p: &std::path::Path) -> bool {
    p.extension().and_then(|e| e.to_str()).is_some_and(|e| ["stl", "obj", "3mf"].iter().any(|k| e.eq_ignore_ascii_case(k)))
}

/// The triangles of a model file, by its extension.
pub fn read(name: &str, bytes: &[u8]) -> Result<Vec<Tri>, String> {
    let lower = name.to_ascii_lowercase();
    let tris = if lower.ends_with(".stl") {
        read_stl(bytes)?
    } else if lower.ends_with(".obj") {
        read_obj(&String::from_utf8_lossy(bytes))?
    } else if lower.ends_with(".3mf") {
        read_3mf(bytes)?
    } else {
        return Err("a model is a .stl, .obj or .3mf file".into());
    };
    if tris.is_empty() {
        return Err("the file has no triangles".into());
    }
    if tris.len() > MAX_TRIANGLES {
        return Err("the model has too many triangles".into());
    }
    if tris.iter().flatten().flatten().any(|v| !v.is_finite()) {
        return Err("the model has a number that is not finite".into());
    }
    Ok(tris)
}

fn read_stl(b: &[u8]) -> Result<Vec<Tri>, String> {
    // Binary: 80 bytes of header, the count, and 50 bytes per triangle. Some ASCII files also start with "solid", so the size decides.
    if b.len() >= 84 {
        let n = u32::from_le_bytes([b[80], b[81], b[82], b[83]]) as usize;
        if n.checked_mul(50).and_then(|k| k.checked_add(84)) == Some(b.len()) {
            let f = |o: usize| f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
            return Ok((0..n)
                .map(|i| {
                    let o = 84 + i * 50 + 12;
                    let v = |k: usize| [f(o + k * 12), f(o + k * 12 + 4), f(o + k * 12 + 8)];
                    [v(0), v(1), v(2)]
                })
                .collect());
        }
    }
    let text = String::from_utf8_lossy(b);
    let mut pts: Vec<[f32; 3]> = vec![];
    for line in text.lines() {
        let mut w = line.split_whitespace();
        if w.next().is_some_and(|k| k.eq_ignore_ascii_case("vertex")) {
            let n: Vec<f32> = w.filter_map(|x| x.parse().ok()).collect();
            if n.len() == 3 {
                pts.push([n[0], n[1], n[2]]);
            }
        }
    }
    if pts.is_empty() {
        return Err("this is not an STL file".into());
    }
    Ok(pts.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect())
}

fn read_obj(text: &str) -> Result<Vec<Tri>, String> {
    let mut v: Vec<[f32; 3]> = vec![];
    let mut tris = vec![];
    for line in text.lines() {
        let mut w = line.split_whitespace();
        match w.next() {
            Some("v") => {
                let n: Vec<f32> = w.take(3).filter_map(|x| x.parse().ok()).collect();
                if n.len() == 3 {
                    v.push([n[0], n[1], n[2]]);
                }
            }
            Some("f") => {
                // "1", "1/2", "1/2/3", "1//3", and negative numbers count from the end.
                let idx: Vec<usize> = w
                    .filter_map(|x| x.split('/').next()?.parse::<i64>().ok())
                    .filter_map(|i| if i > 0 { Some(i as usize - 1) } else if i < 0 { v.len().checked_sub((-i) as usize) } else { None })
                    .filter(|i| *i < v.len())
                    .collect();
                for k in 2..idx.len() {
                    tris.push([v[idx[0]], v[idx[k - 1]], v[idx[k]]]);
                }
            }
            _ => {}
        }
    }
    if tris.is_empty() {
        return Err("this OBJ file has no faces".into());
    }
    Ok(tris)
}

/// The value of an attribute (`x="1.5"`) in the text of a tag.
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!("{name}=\"");
    let at = tag.find(&key)?;
    // The attribute has to start the word: "x=" is not the end of "dx=".
    if at > 0 && !tag.as_bytes()[at - 1].is_ascii_whitespace() {
        return None;
    }
    let rest = &tag[at + key.len()..];
    Some(&rest[..rest.find('"')?])
}

fn read_3mf(b: &[u8]) -> Result<Vec<Tri>, String> {
    use std::io::Read;
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(b)).map_err(|e| format!("this is not a 3MF file: {e}"))?;
    let names: Vec<String> = (0..zip.len()).filter_map(|i| zip.by_index(i).ok().and_then(|f| f.name().ok().map(|n| n.to_string()))).filter(|n| n.to_ascii_lowercase().ends_with(".model")).collect();
    if names.is_empty() {
        return Err("this 3MF file has no model".into());
    }
    let mut tris = vec![];
    for name in names {
        let mut xml = String::new();
        // The size of the unpacked file is limited (a zip bomb).
        zip.by_name(&name).map_err(|e| e.to_string())?.take(256 * 1024 * 1024).read_to_string(&mut xml).map_err(|e| e.to_string())?;
        // The vertices and triangles of every object; the indices of a triangle count in its own object.
        for object in xml.split("<object").skip(1) {
            let object = object.split("</object>").next().unwrap_or("");
            let mut verts: Vec<[f32; 3]> = vec![];
            for tag in object.split("<vertex").skip(1) {
                let tag = tag.split('>').next().unwrap_or("");
                let n = |k: &str| attr(tag, k).and_then(|x| x.parse::<f32>().ok());
                if let (Some(x), Some(y), Some(z)) = (n("x"), n("y"), n("z")) {
                    verts.push([x, y, z]);
                }
            }
            for tag in object.split("<triangle").skip(1) {
                let tag = tag.split('>').next().unwrap_or("");
                let n = |k: &str| attr(tag, k).and_then(|x| x.parse::<usize>().ok()).and_then(|i| verts.get(i).copied());
                if let (Some(a), Some(b), Some(c)) = (n("v1"), n("v2"), n("v3")) {
                    tris.push([a, b, c]);
                }
            }
        }
    }
    if tris.is_empty() {
        return Err("this 3MF file has no triangles".into());
    }
    Ok(tris)
}

/// The size of the model: its width and depth (x and y) and its height (z), in mm.
pub fn size(tris: &[Tri]) -> [f64; 3] {
    let (lo, hi) = bounds(tris);
    [(hi[0] - lo[0]) as f64, (hi[1] - lo[1]) as f64, (hi[2] - lo[2]) as f64]
}

fn bounds(tris: &[Tri]) -> ([f32; 3], [f32; 3]) {
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for v in tris.iter().flatten() {
        for k in 0..3 {
            lo[k] = lo[k].min(v[k]);
            hi[k] = hi[k].max(v[k]);
        }
    }
    (lo, hi)
}

/// The depth map of the model seen from above, with pixels of about `px_mm` (coarser when the model is big). The picture is as big
/// as the model in mm. `None` when the model is flat or has no area.
pub fn depth_map(tris: &[Tri], name: &str, px_mm: f64) -> Option<ImageData> {
    let (lo, hi) = bounds(tris);
    let (w, h, zr) = ((hi[0] - lo[0]) as f64, (hi[1] - lo[1]) as f64, (hi[2] - lo[2]) as f64);
    if w < 1e-6 || h < 1e-6 {
        return None;
    }
    let px = px_mm.max(0.01).max(w.max(h) / MAX_SIDE);
    let (pw, ph) = (((w / px).ceil() as usize).max(1), ((h / px).ceil() as usize).max(1));
    let mut z = vec![f32::NEG_INFINITY; pw * ph];
    // The row 0 of the picture is the top of the page: the largest y of the model.
    let (sx, sy) = ((pw as f64 / w) as f32, (ph as f64 / h) as f32);
    for t in tris {
        let p: Vec<[f32; 3]> = t.iter().map(|v| [(v[0] - lo[0]) * sx, (hi[1] - v[1]) * sy, v[2]]).collect();
        let (x0, x1) = (p.iter().map(|q| q[0]).fold(f32::INFINITY, f32::min), p.iter().map(|q| q[0]).fold(f32::NEG_INFINITY, f32::max));
        let (y0, y1) = (p.iter().map(|q| q[1]).fold(f32::INFINITY, f32::min), p.iter().map(|q| q[1]).fold(f32::NEG_INFINITY, f32::max));
        let area = (p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[2][0] - p[0][0]) * (p[1][1] - p[0][1]);
        if area.abs() < 1e-9 {
            continue;
        }
        let (c0, c1) = ((x0 - 0.5).floor().max(0.0) as usize, ((x1 + 0.5).ceil().max(0.0) as usize).min(pw));
        let (r0, r1) = ((y0 - 0.5).floor().max(0.0) as usize, ((y1 + 0.5).ceil().max(0.0) as usize).min(ph));
        for r in r0..r1 {
            for c in c0..c1 {
                let (qx, qy) = (c as f32 + 0.5, r as f32 + 0.5);
                let w0 = ((p[1][0] - qx) * (p[2][1] - qy) - (p[2][0] - qx) * (p[1][1] - qy)) / area;
                let w1 = ((p[2][0] - qx) * (p[0][1] - qy) - (p[0][0] - qx) * (p[2][1] - qy)) / area;
                let w2 = 1.0 - w0 - w1;
                let eps = -1e-4;
                if w0 >= eps && w1 >= eps && w2 >= eps {
                    let h = w0 * p[0][2] + w1 * p[1][2] + w2 * p[2][2];
                    let cell = &mut z[r * pw + c];
                    if h > *cell {
                        *cell = h;
                    }
                }
            }
        }
    }
    let gray: Vec<u8> = z.iter().map(|h| if h.is_finite() && zr > 1e-9 { (((h - lo[2]) as f64 / zr) * 255.0).round().clamp(0.0, 255.0) as u8 } else if h.is_finite() { 255 } else { 0 }).collect();
    Some(ImageData { name: name.to_string(), px_w: pw as u32, px_h: ph as u32, gray, w, h, invert: false })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A wedge 10 x 10 mm that rises from 0 at x = 0 to 5 mm at x = 10, as a closed set of triangles (top and the two sides are enough).
    fn wedge() -> Vec<Tri> {
        let a = [0.0, 0.0, 0.0];
        let b = [10.0, 0.0, 5.0];
        let c = [10.0, 10.0, 5.0];
        let d = [0.0, 10.0, 0.0];
        vec![[a, b, c], [a, c, d]]
    }

    fn binary_stl(tris: &[Tri]) -> Vec<u8> {
        let mut out = vec![0u8; 80];
        out.extend((tris.len() as u32).to_le_bytes());
        for t in tris {
            out.extend([0u8; 12]);
            for v in t {
                for x in v {
                    out.extend(x.to_le_bytes());
                }
            }
            out.extend([0u8; 2]);
        }
        out
    }

    #[test]
    fn a_ramp_becomes_a_depth_map_that_is_black_where_it_is_low_and_white_where_it_is_high() {
        let im = depth_map(&wedge(), "wedge", 0.5).unwrap();
        assert_eq!((im.w, im.h), (10.0, 10.0));
        let (w, h) = (im.px_w as usize, im.px_h as usize);
        let at = |c: usize, r: usize| im.gray[r * w + c];
        assert!(at(0, h / 2) < 25, "low at the left: {}", at(0, h / 2));
        assert!(at(w - 1, h / 2) > 230, "high at the right: {}", at(w - 1, h / 2));
        assert!((at(w / 2, h / 2) as i32 - 128).abs() < 20, "half way in the middle");
    }

    #[test]
    fn where_the_model_has_nothing_the_map_is_black_and_the_top_of_the_picture_is_the_back_of_the_model() {
        // One triangle in the corner of a box 10 x 10: the rest of the box is empty.
        let tris: Vec<Tri> = vec![[[0.0, 0.0, 3.0], [4.0, 0.0, 3.0], [0.0, 4.0, 3.0]], [[10.0, 10.0, 0.0], [10.0, 9.9, 0.0], [9.9, 10.0, 0.0]]];
        let im = depth_map(&tris, "t", 0.5).unwrap();
        let (w, h) = (im.px_w as usize, im.px_h as usize);
        // The model's front-left corner (y = 0) is at the bottom of the picture.
        assert_eq!(im.gray[(h - 2) * w + 1], 255, "the high triangle is white");
        assert_eq!(im.gray[w / 2], 0, "empty is black");
    }

    #[test]
    fn stl_binary_and_ascii_and_obj_give_the_same_triangles() {
        let tris = wedge();
        let bin = read("a.stl", &binary_stl(&tris)).unwrap();
        assert_eq!(bin, tris);
        let ascii = "solid x\nfacet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 10 0 5\nvertex 10 10 5\nendloop\nendfacet\nfacet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 10 10 5\nvertex 0 10 0\nendloop\nendfacet\nendsolid x\n";
        assert_eq!(read("a.STL", ascii.as_bytes()).unwrap(), tris);
        let obj = "v 0 0 0\nv 10 0 5\nv 10 10 5\nv 0 10 0\nf 1 2 3\nf 1/1/1 3/3/3 4/4/4\n";
        assert_eq!(read("a.obj", obj.as_bytes()).unwrap(), tris);
        // A quad is two triangles, negative indices count from the end.
        let quad = "v 0 0 0\nv 10 0 5\nv 10 10 5\nv 0 10 0\nf -4 -3 -2 -1\n";
        assert_eq!(read("q.obj", quad.as_bytes()).unwrap().len(), 2);
    }

    #[test]
    fn a_3mf_file_is_read_from_its_zip_and_bad_files_are_refused() {
        use std::io::Write;
        let xml = r#"<?xml version="1.0"?><model><resources><object id="1" type="model"><mesh><vertices><vertex x="0" y="0" z="0"/><vertex x="10" y="0" z="5"/><vertex x="10" y="10" z="5"/><vertex x="0" y="10" z="0"/></vertices><triangles><triangle v1="0" v2="1" v3="2"/><triangle v1="0" v2="2" v3="3"/></triangles></mesh></object></resources></model>"#;
        let mut buf = std::io::Cursor::new(vec![]);
        {
            let mut z = zip::ZipWriter::new(&mut buf);
            z.start_file("3D/3dmodel.model", zip::write::SimpleFileOptions::default()).unwrap();
            z.write_all(xml.as_bytes()).unwrap();
            z.finish().unwrap();
        }
        assert_eq!(read("m.3mf", &buf.into_inner()).unwrap(), wedge());
        assert!(read("m.3mf", b"not a zip").is_err());
        assert!(read("m.stl", b"nothing").is_err());
        assert!(read("m.obj", b"v 1 2 3\n").is_err());
        assert!(read("m.txt", b"").is_err());
        assert!(depth_map(&[[[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 2.0]]], "flat", 0.5).is_none(), "a model with no area seen from above");
    }

    #[test]
    fn size_is_the_box_of_the_model() {
        assert_eq!(size(&wedge()), [10.0, 10.0, 5.0]);
        assert!(is_model_file(std::path::Path::new("a/B.STL")) && is_model_file(std::path::Path::new("x.3mf")) && !is_model_file(std::path::Path::new("x.png")));
    }
}
