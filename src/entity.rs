use crate::*;
use disk_types::*;

pub struct DeviceEntity<'a> {
    pub(crate) ctx: &'a DiskManager,
    pub id:         Entity,
}

impl<'a> DeviceEntity<'a> {
    // If the device is a loopback, this will kdisplay the backing file.
    pub fn backing_file(&self) -> Option<&Path> {
        self.ctx.components.loopbacks.get(self.id).map(AsRef::as_ref)
    }

    // Provides an iterator for listing children of a device, for devices that support having
    // multiple children.
    pub fn children(&self) -> impl Iterator<Item = DeviceEntity<'_>> {
        self.ctx
            .components
            .children
            .get(self.id)
            .into_iter()
            .flat_map(|entities| entities.iter())
            .map(move |&id| DeviceEntity { id, ctx: self.ctx })
    }

    // Access information about this device.
    pub fn device(&self) -> &Device { self.ctx.device(self.id) }

    // If the device is a device map, this will return its name.
    pub fn device_map_name(&self) -> Option<&str> {
        self.ctx.components.device_maps.get(self.id).map(AsRef::as_ref)
    }

    // If the device is a disk, information about that disk can be retrieved here.
    pub fn disk(&self) -> Option<&Disk> { self.ctx.components.disks.get(self.id) }

    // For LV devices which are associated with a VG.
    pub fn lv(&self) -> Option<(&LvmVg, &LvmLv)> {
        self.ctx
            .components
            .lvs
            .get(self.id)
            .map(|(lv, vg_entity)| (self.ctx.components.vgs.get(*vg_entity), lv))
    }

    // For PVs which may be associated with a VG.
    pub fn pv<'b>(&'b self) -> Option<(Option<&'a LvmVg>, &'b LvmPv)> {
        self.ctx.components.pvs.get(self.id).map(|(pv, vg_entity)| {
            let vg = vg_entity.map(|ent| self.ctx.components.vgs.get(ent));
            (vg, pv)
        })
    }

    // If the device is a LUKS partition, information about the LUKS device is here.
    pub fn luks(&self) -> Option<&Luks> { self.ctx.components.luks.get(self.id) }

    // Return the parent of this device, if this device has one.
    pub fn parents(&self) -> impl Iterator<Item = DeviceEntity<'_>> {
        self.ctx
            .components
            .parents
            .get(self.id)
            .into_iter()
            .flat_map(|entities| entities.iter())
            .map(move |&id| DeviceEntity { id, ctx: self.ctx })
    }

    pub fn partition<'b>(&'b self) -> Option<&'a Partition> {
        self.ctx.components.partitions.get(self.id)
    }
}
