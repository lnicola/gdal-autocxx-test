#pragma once

#include <gdal/gdal.h>
#include <gdal/gdal_priv.h>

inline GDALMajorObject *upcast_driver(GDALDriver *driver) { return driver; }
