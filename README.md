## Building

`autocxx` doesn't support `GDALDriver::{AddFieldDomain, UpdateFieldDomain, AddRelationship, UpdateRelationship}`, they have to be patched out of `gdal_priv.h`.
