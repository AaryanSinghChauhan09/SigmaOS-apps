# SigmaOS-apps

Curated application defaults, integrations, and application profiles for the
SigmaOS product profile. Applications remain replaceable components and must
not become kernel dependencies.

## Current state

`COMPONENT.toml` marks this repository as a scaffold. The current checkout
contains the component manifest, license, README, and metadata validation
workflow; it does not yet define an application catalog, profiles, or install
integration.

## Intended boundary

Keep product selection and application metadata separate from kernel and core
system services. Each future entry should identify its upstream, package or
installation source, supported profile, and removal or replacement path.
Installation and update mechanisms belong to the package-management boundary;
this repository should describe selection and integration policy rather than
silently take ownership of package execution.

## Readiness evidence

Before describing a curated profile as available, add machine-readable
entries, validate them against the package interface, and document the
resulting install and removal behavior. The current metadata workflow validates
component declarations only; it does not install applications.
