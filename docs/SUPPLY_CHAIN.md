# Supply chain

The initial production dependency closure is exactly the Rust standard
library. The workspace contains no Cargo dependency, build script, native
library, FFI surface, vendored source, runtime feature, network client, or
publication job. The lockfile records only the local facade package.

Changing that state requires a reviewed work unit that names the dependency or
tool, its exact purpose and enabled features, its license and provenance, its
maintenance and security posture, and the validation and rollback effects.
Dependencies must not be added speculatively.

GitHub Actions used for source checkout or toolchain setup are CI tooling, not
production dependencies. Workflow revisions still require review and must not
gain publishing credentials or release permissions as part of an ordinary
validation change.
