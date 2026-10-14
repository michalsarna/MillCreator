# Security policy

## Supported version

Only the newest released version receives security fixes.

| Version | Supported |
|---------|-----------|
| v0.2.1 (latest) | yes |
| v0.2.0 | no, please update |
| older | no, please update |

## Reporting a vulnerability

Please do not open a public issue for a security problem. Use GitHub's private vulnerability reporting:
<https://github.com/michalsarna/MillCreator/security/advisories/new>

Include the version, your operating system, what you did and what happened. Files that trigger the problem
(an SVG, PDF / Illustrator or image) are very helpful. You can expect an answer within a few days; a fix is released
as a new version.

## What counts

MillCreator opens files from other people (SVG, PDF / Illustrator, LightBurn, `.cre` projects, device and bit files) and
writes G-code for machines. Reports about crashes, hangs or memory use when opening a crafted file, and about anything
that could write unintended G-code, are in scope. Problems in the shared code are handled in
[baseCreator](https://github.com/michalsarna/baseCreator/security/advisories/new).

## Safety reminder

A milling machine is dangerous. MillCreator writes G-code and does not control the machine; test every job in the air
first, wear eye and ear protection and never leave a running machine unattended.
