## Building

`autocxx` doesn't support `GDALDriver::{AddFieldDomain, UpdateFieldDomain, AddRelationship, UpdateRelationship}`, they have to be patched out of `gdal_priv.h`.

Set the library search path in `build.rs` if required.

```bash
$ cargo build
$ cp target/debug/libgdal_autocxx_test.so target/debug/gdal_autocxx_test.so
$ GDAL_DRIVER_PATH=target/debug gdalinfo --formats
```
