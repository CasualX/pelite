/*!
These Proof of Concept PE files are borrowed from [corkami's pocs](https://github.com/corkami/pocs).

Some antivirus flip when they see these binaries, so they're concatenated them in a single blob.

This code is run as part of the library's unit tests, see `src/tests.rs`.
 */

pub fn iter() -> impl Iterator<Item = (&'static str, &'static [u8])> {
    BINS.iter().map(|bin| {
        let start = bin.offset as usize;
        let end = start + bin.len as usize;
        let bytes = &BLOB[start..end];
        (bin.name, bytes)
    })
}

static BLOB: [u8; 0x46670] = *include_bytes!("pocs.blob");

#[derive(Copy, Clone, Debug)]
struct Binary {
    offset: u32,
    len: u32,
    name: &'static str,
}

static BINS: [Binary; 217] = [
    Binary {
        offset: 0x10,
        len: 0x1400,
        name: "96emptysections.exe",
    },
    Binary {
        offset: 0x1410,
        len: 0x494,
        name: "appendeddata.exe",
    },
    Binary {
        offset: 0x18b0,
        len: 0x520,
        name: "appendedhdr.exe",
    },
    Binary {
        offset: 0x1dd0,
        len: 0x428,
        name: "appendedsecttbl.exe",
    },
    Binary {
        offset: 0x2200,
        len: 0x520,
        name: "apphdrW7.exe",
    },
    Binary {
        offset: 0x2720,
        len: 0x428,
        name: "appsectableW7.exe",
    },
    Binary {
        offset: 0x2b50,
        len: 0x400,
        name: "aslr-ld.exe",
    },
    Binary {
        offset: 0x2f50,
        len: 0x400,
        name: "aslr.dll",
    },
    Binary {
        offset: 0x3350,
        len: 0x400,
        name: "bigib.exe",
    },
    Binary {
        offset: 0x3750,
        len: 0x400,
        name: "bigsec.exe",
    },
    Binary {
        offset: 0x3b50,
        len: 0x600,
        name: "bigSoRD.exe",
    },
    Binary {
        offset: 0x4150,
        len: 0x400,
        name: "bottomsecttbl.exe",
    },
    Binary {
        offset: 0x4550,
        len: 0x400,
        name: "cfgbogus.exe",
    },
    Binary {
        offset: 0x4950,
        len: 0xa00,
        name: "compiled.exe",
    },
    Binary {
        offset: 0x5350,
        len: 0x600,
        name: "copyright.exe",
    },
    Binary {
        offset: 0x5950,
        len: 0x400,
        name: "ctxt-ld.exe",
    },
    Binary {
        offset: 0x5d50,
        len: 0x400,
        name: "ctxt.dll",
    },
    Binary {
        offset: 0x6150,
        len: 0x400,
        name: "ddsect.exe",
    },
    Binary {
        offset: 0x6550,
        len: 0x600,
        name: "debug.exe",
    },
    Binary {
        offset: 0x6b50,
        len: 0x400,
        name: "delaycorrupt.exe",
    },
    Binary {
        offset: 0x6f50,
        len: 0x400,
        name: "delayfake.exe",
    },
    Binary {
        offset: 0x7350,
        len: 0x400,
        name: "delayimports.exe",
    },
    Binary {
        offset: 0x7750,
        len: 0x400,
        name: "dep.exe",
    },
    Binary {
        offset: 0x7b50,
        len: 0x400,
        name: "dll-dynld.exe",
    },
    Binary {
        offset: 0x7f50,
        len: 0x400,
        name: "dll-dynunicld.exe",
    },
    Binary {
        offset: 0x8350,
        len: 0x400,
        name: "dll-ld.exe",
    },
    Binary {
        offset: 0x8750,
        len: 0x400,
        name: "dll-webdavld.exe",
    },
    Binary {
        offset: 0x8b50,
        len: 0x400,
        name: "dll.dll",
    },
    Binary {
        offset: 0x8f50,
        len: 0x400,
        name: "dllbound-ld.exe",
    },
    Binary {
        offset: 0x9350,
        len: 0x400,
        name: "dllbound-redirld.exe",
    },
    Binary {
        offset: 0x9750,
        len: 0x400,
        name: "dllbound-redirldXP.exe",
    },
    Binary {
        offset: 0x9b50,
        len: 0x400,
        name: "dllbound.dll",
    },
    Binary {
        offset: 0x9f50,
        len: 0x400,
        name: "dllbound2.dll",
    },
    Binary {
        offset: 0xa350,
        len: 0x400,
        name: "dllcfgdup-dynld.exe",
    },
    Binary {
        offset: 0xa750,
        len: 0x600,
        name: "dllcfgdup.dll",
    },
    Binary {
        offset: 0xad50,
        len: 0x400,
        name: "dllemptyexp-ld.exe",
    },
    Binary {
        offset: 0xb150,
        len: 0x400,
        name: "dllemptyexp.dll",
    },
    Binary {
        offset: 0xb550,
        len: 0x400,
        name: "dllextep-ld.exe",
    },
    Binary {
        offset: 0xb950,
        len: 0x400,
        name: "dllextep.dll",
    },
    Binary {
        offset: 0xbd50,
        len: 0x400,
        name: "dllfakess-dynld.exe",
    },
    Binary {
        offset: 0xc150,
        len: 0x400,
        name: "dllfakess-ld.exe",
    },
    Binary {
        offset: 0xc550,
        len: 0x400,
        name: "dllfakess.dll",
    },
    Binary {
        offset: 0xc950,
        len: 0x400,
        name: "dllfw-ld.exe",
    },
    Binary {
        offset: 0xcd50,
        len: 0x400,
        name: "dllfw.dll",
    },
    Binary {
        offset: 0xd150,
        len: 0x400,
        name: "dllfwloop-ld.exe",
    },
    Binary {
        offset: 0xd550,
        len: 0x400,
        name: "dllfwloop.dll",
    },
    Binary {
        offset: 0xd950,
        len: 0x400,
        name: "dllmaxvals-dynld.exe",
    },
    Binary {
        offset: 0xdd50,
        len: 0x400,
        name: "dllmaxvals-ld.exe",
    },
    Binary {
        offset: 0xe150,
        len: 0x400,
        name: "dllmaxvals.dll",
    },
    Binary {
        offset: 0xe550,
        len: 0x400,
        name: "dllnegep-ld.exe",
    },
    Binary {
        offset: 0xe950,
        len: 0x400,
        name: "dllnegep.dll",
    },
    Binary {
        offset: 0xed50,
        len: 0x400,
        name: "dllnoexp-dynld.exe",
    },
    Binary {
        offset: 0xf150,
        len: 0x400,
        name: "dllnoexp.dll",
    },
    Binary {
        offset: 0xf550,
        len: 0x400,
        name: "dllnomain-ld.exe",
    },
    Binary {
        offset: 0xf950,
        len: 0x400,
        name: "dllnomain.dll",
    },
    Binary {
        offset: 0xfd50,
        len: 0x400,
        name: "dllnomain2-dynld.exe",
    },
    Binary {
        offset: 0x10150,
        len: 0x400,
        name: "dllnomain2.dll",
    },
    Binary {
        offset: 0x10550,
        len: 0x400,
        name: "dllnoreloc-ld.exe",
    },
    Binary {
        offset: 0x10950,
        len: 0x400,
        name: "dllnoreloc.dll",
    },
    Binary {
        offset: 0x10d50,
        len: 0x400,
        name: "dllnullep-dynld.exe",
    },
    Binary {
        offset: 0x11150,
        len: 0x400,
        name: "dllnullep-ld.exe",
    },
    Binary {
        offset: 0x11550,
        len: 0x400,
        name: "dllnullep.dll",
    },
    Binary {
        offset: 0x11950,
        len: 0x400,
        name: "dllord-ld.exe",
    },
    Binary {
        offset: 0x11d50,
        len: 0x400,
        name: "dllord.dll",
    },
    Binary {
        offset: 0x12150,
        len: 0x40,
        name: "dosZMXP.exe",
    },
    Binary {
        offset: 0x12190,
        len: 0xc00,
        name: "dotnet20.exe",
    },
    Binary {
        offset: 0x12d90,
        len: 0x400,
        name: "driver.sys",
    },
    Binary {
        offset: 0x13190,
        len: 0x400,
        name: "dump_imports.exe",
    },
    Binary {
        offset: 0x13590,
        len: 0x800,
        name: "duphead.exe",
    },
    Binary {
        offset: 0x13d90,
        len: 0x400,
        name: "dupsec.exe",
    },
    Binary {
        offset: 0x14190,
        len: 0x400,
        name: "d_resource-ld.exe",
    },
    Binary {
        offset: 0x14590,
        len: 0x280,
        name: "d_resource.dll",
    },
    Binary {
        offset: 0x14810,
        len: 0x400,
        name: "d_tiny-ld.exe",
    },
    Binary {
        offset: 0x14c10,
        len: 0x3d,
        name: "d_tiny.dll",
    },
    Binary {
        offset: 0x14c50,
        len: 0x400,
        name: "exceptions.exe",
    },
    Binary {
        offset: 0x15050,
        len: 0xa00,
        name: "exe2pe.exe",
    },
    Binary {
        offset: 0x15a50,
        len: 0x400,
        name: "exportobf.exe",
    },
    Binary {
        offset: 0x15e50,
        len: 0x400,
        name: "exportsdata.exe",
    },
    Binary {
        offset: 0x16250,
        len: 0x400,
        name: "exports_doc.exe",
    },
    Binary {
        offset: 0x16650,
        len: 0x400,
        name: "exports_order.exe",
    },
    Binary {
        offset: 0x16a50,
        len: 0x600,
        name: "fakenet.exe",
    },
    Binary {
        offset: 0x17050,
        len: 0x600,
        name: "fakeregs.exe",
    },
    Binary {
        offset: 0x17650,
        len: 0x400,
        name: "fakeregslib.dll",
    },
    Binary {
        offset: 0x17a50,
        len: 0x400,
        name: "fakerelocs.exe",
    },
    Binary {
        offset: 0x17e50,
        len: 0x10a0,
        name: "foldedhdr.exe",
    },
    Binary {
        offset: 0x18ef0,
        len: 0x10a0,
        name: "foldedhdrW7.exe",
    },
    Binary {
        offset: 0x19f90,
        len: 0x520,
        name: "footer.exe",
    },
    Binary {
        offset: 0x1a4b0,
        len: 0x400,
        name: "gui.exe",
    },
    Binary {
        offset: 0x1a8b0,
        len: 0x400,
        name: "hard_imports.exe",
    },
    Binary {
        offset: 0x1acb0,
        len: 0x10c,
        name: "hdrcode.exe",
    },
    Binary {
        offset: 0x1adc0,
        len: 0x400,
        name: "hdrdata.exe",
    },
    Binary {
        offset: 0x1b1c0,
        len: 0x601,
        name: "hiddenappdata1.exe",
    },
    Binary {
        offset: 0x1b7d0,
        len: 0x494,
        name: "hiddenappdata2.exe",
    },
    Binary {
        offset: 0x1bc70,
        len: 0x400,
        name: "ibkernel.exe",
    },
    Binary {
        offset: 0x1c070,
        len: 0x400,
        name: "ibkmanual.exe",
    },
    Binary {
        offset: 0x1c470,
        len: 0x400,
        name: "ibknoreloc64.exe",
    },
    Binary {
        offset: 0x1c870,
        len: 0x400,
        name: "ibnullXP.exe",
    },
    Binary {
        offset: 0x1cc70,
        len: 0x1000,
        name: "ibreloc.exe",
    },
    Binary {
        offset: 0x1dc70,
        len: 0x1000,
        name: "ibrelocW7.exe",
    },
    Binary {
        offset: 0x1ec70,
        len: 0x400,
        name: "impbyord.exe",
    },
    Binary {
        offset: 0x1f070,
        len: 0x400,
        name: "imports.exe",
    },
    Binary {
        offset: 0x1f470,
        len: 0x400,
        name: "importsdotXP.exe",
    },
    Binary {
        offset: 0x1f870,
        len: 0x400,
        name: "importshint.exe",
    },
    Binary {
        offset: 0x1fc70,
        len: 0x400,
        name: "imports_apimsW7.exe",
    },
    Binary {
        offset: 0x20070,
        len: 0x400,
        name: "imports_badterm.exe",
    },
    Binary {
        offset: 0x20470,
        len: 0x400,
        name: "imports_bogusIAT.exe",
    },
    Binary {
        offset: 0x20870,
        len: 0x400,
        name: "imports_corruptedIAT.exe",
    },
    Binary {
        offset: 0x20c70,
        len: 0x400,
        name: "imports_iatindesc.exe",
    },
    Binary {
        offset: 0x21070,
        len: 0x400,
        name: "imports_mixed.exe",
    },
    Binary {
        offset: 0x21470,
        len: 0x400,
        name: "imports_multidesc.exe",
    },
    Binary {
        offset: 0x21870,
        len: 0x400,
        name: "imports_nnIAT.exe",
    },
    Binary {
        offset: 0x21c70,
        len: 0x400,
        name: "imports_noext.exe",
    },
    Binary {
        offset: 0x22070,
        len: 0x400,
        name: "imports_noint.exe",
    },
    Binary {
        offset: 0x22470,
        len: 0x400,
        name: "imports_relocW7.exe",
    },
    Binary {
        offset: 0x22870,
        len: 0x400,
        name: "imports_tinyW7.exe",
    },
    Binary {
        offset: 0x22c70,
        len: 0x400,
        name: "imports_tinyXP.exe",
    },
    Binary {
        offset: 0x23070,
        len: 0x400,
        name: "imports_virtdesc.exe",
    },
    Binary {
        offset: 0x23470,
        len: 0x400,
        name: "imports_vterm.exe",
    },
    Binary {
        offset: 0x23870,
        len: 0x400,
        name: "ldrsnaps.exe",
    },
    Binary {
        offset: 0x23c70,
        len: 0x600,
        name: "ldrsnaps64.exe",
    },
    Binary {
        offset: 0x24270,
        len: 0x1100,
        name: "lowaldiff.exe",
    },
    Binary {
        offset: 0x25370,
        len: 0x400,
        name: "lowsubsys.exe",
    },
    Binary {
        offset: 0x25770,
        len: 0x600,
        name: "manifest.exe",
    },
    Binary {
        offset: 0x25d70,
        len: 0x400,
        name: "manifest_broken.exe",
    },
    Binary {
        offset: 0x26170,
        len: 0x800,
        name: "manifest_bsod.exe",
    },
    Binary {
        offset: 0x26970,
        len: 0x1147,
        name: "maxsecXP.exe",
    },
    Binary {
        offset: 0x27ac0,
        len: 0x400,
        name: "maxvals.exe",
    },
    Binary {
        offset: 0x27ec0,
        len: 0x400,
        name: "memshared-ld.exe",
    },
    Binary {
        offset: 0x282c0,
        len: 0x400,
        name: "memshared.dll",
    },
    Binary {
        offset: 0x286c0,
        len: 0x148,
        name: "mini.exe",
    },
    Binary {
        offset: 0x28810,
        len: 0x400,
        name: "mscoree.exe",
    },
    Binary {
        offset: 0x28c10,
        len: 0x600,
        name: "multiss.exe",
    },
    Binary {
        offset: 0x29210,
        len: 0x600,
        name: "multiss_con.exe",
    },
    Binary {
        offset: 0x29810,
        len: 0x600,
        name: "multiss_drv.sys",
    },
    Binary {
        offset: 0x29e10,
        len: 0x600,
        name: "multiss_gui.exe",
    },
    Binary {
        offset: 0x2a410,
        len: 0x400,
        name: "mz.exe",
    },
    Binary {
        offset: 0x2a810,
        len: 0x400,
        name: "namedresource.exe",
    },
    Binary {
        offset: 0x2ac10,
        len: 0x400,
        name: "normal.exe",
    },
    Binary {
        offset: 0x2b010,
        len: 0x400,
        name: "normal64.exe",
    },
    Binary {
        offset: 0x2b410,
        len: 0x240,
        name: "nosectionW7.exe",
    },
    Binary {
        offset: 0x2b650,
        len: 0x237,
        name: "nosectionXP.exe",
    },
    Binary {
        offset: 0x2b890,
        len: 0x400,
        name: "nothing-ld.exe",
    },
    Binary {
        offset: 0x2bc90,
        len: 0x310,
        name: "nothing.dll",
    },
    Binary {
        offset: 0x2bfa0,
        len: 0x400,
        name: "no_dd.exe",
    },
    Binary {
        offset: 0x2c3a0,
        len: 0x400,
        name: "no_dd64.exe",
    },
    Binary {
        offset: 0x2c7a0,
        len: 0x400,
        name: "no_dep.exe",
    },
    Binary {
        offset: 0x2cba0,
        len: 0x400,
        name: "no_seh.exe",
    },
    Binary {
        offset: 0x2cfa0,
        len: 0x400,
        name: "nullEP.exe",
    },
    Binary {
        offset: 0x2d3a0,
        len: 0x247,
        name: "nullSOH-XP.exe",
    },
    Binary {
        offset: 0x2d5f0,
        len: 0x400,
        name: "nullvirt.exe",
    },
    Binary {
        offset: 0x2d9f0,
        len: 0x400,
        name: "ownexports.exe",
    },
    Binary {
        offset: 0x2ddf0,
        len: 0x400,
        name: "ownexports2.exe",
    },
    Binary {
        offset: 0x2e1f0,
        len: 0x400,
        name: "ownexportsdot.exe",
    },
    Binary {
        offset: 0x2e5f0,
        len: 0x2f0,
        name: "pdf.exe",
    },
    Binary {
        offset: 0x2e8e0,
        len: 0x413,
        name: "pdf_zip_pe.exe",
    },
    Binary {
        offset: 0x2ed00,
        len: 0x20b6,
        name: "quine.exe",
    },
    Binary {
        offset: 0x30dc0,
        len: 0x400,
        name: "reloc4.exe",
    },
    Binary {
        offset: 0x311c0,
        len: 0x400,
        name: "reloc9.exe",
    },
    Binary {
        offset: 0x315c0,
        len: 0x800,
        name: "reloccrypt.exe",
    },
    Binary {
        offset: 0x31dc0,
        len: 0x800,
        name: "reloccryptW8.exe",
    },
    Binary {
        offset: 0x325c0,
        len: 0xc00,
        name: "reloccryptXP.exe",
    },
    Binary {
        offset: 0x331c0,
        len: 0x1200,
        name: "relocOSdet.exe",
    },
    Binary {
        offset: 0x343c0,
        len: 0x400,
        name: "relocsstripped.exe",
    },
    Binary {
        offset: 0x347c0,
        len: 0x400,
        name: "relocsstripped64.exe",
    },
    Binary {
        offset: 0x34bc0,
        len: 0x400,
        name: "reshdr.exe",
    },
    Binary {
        offset: 0x34fc0,
        len: 0x400,
        name: "resource.exe",
    },
    Binary {
        offset: 0x353c0,
        len: 0x400,
        name: "resource2.exe",
    },
    Binary {
        offset: 0x357c0,
        len: 0x400,
        name: "resourceloop.exe",
    },
    Binary {
        offset: 0x35bc0,
        len: 0x1c00,
        name: "resource_icon.exe",
    },
    Binary {
        offset: 0x377c0,
        len: 0x600,
        name: "resource_string.exe",
    },
    Binary {
        offset: 0x37dc0,
        len: 0x400,
        name: "safeseh.exe",
    },
    Binary {
        offset: 0x381c0,
        len: 0x400,
        name: "safeseh_fly.exe",
    },
    Binary {
        offset: 0x385c0,
        len: 0x10c,
        name: "sc.exe",
    },
    Binary {
        offset: 0x386d0,
        len: 0x800,
        name: "secinsec.exe",
    },
    Binary {
        offset: 0x38ed0,
        len: 0x400,
        name: "seh_change64.exe",
    },
    Binary {
        offset: 0x392d0,
        len: 0x600,
        name: "shuffledsect.exe",
    },
    Binary {
        offset: 0x398d0,
        len: 0xc80,
        name: "signature.exe",
    },
    Binary {
        offset: 0x3a550,
        len: 0x400,
        name: "skippeddynbase.exe",
    },
    Binary {
        offset: 0x3a950,
        len: 0x800,
        name: "slackspace.exe",
    },
    Binary {
        offset: 0x3b150,
        len: 0x400,
        name: "ss63.exe",
    },
    Binary {
        offset: 0x3b550,
        len: 0x400,
        name: "ss63nocookie.exe",
    },
    Binary {
        offset: 0x3b950,
        len: 0x2a80,
        name: "standard.exe",
    },
    Binary {
        offset: 0x3e3d0,
        len: 0x10c,
        name: "tiny.exe",
    },
    Binary {
        offset: 0x3e4e0,
        len: 0x400,
        name: "tinydll-ld.exe",
    },
    Binary {
        offset: 0x3e8e0,
        len: 0x11c,
        name: "tinydll.dll",
    },
    Binary {
        offset: 0x3ea00,
        len: 0x400,
        name: "tinydllXP-ld.exe",
    },
    Binary {
        offset: 0x3ee00,
        len: 0x61,
        name: "tinydllXP.dll",
    },
    Binary {
        offset: 0x3ee70,
        len: 0x61,
        name: "tinydrivXP.sys",
    },
    Binary {
        offset: 0x3eee0,
        len: 0x10c,
        name: "tinygui.exe",
    },
    Binary {
        offset: 0x3eff0,
        len: 0x600,
        name: "tinynet.exe",
    },
    Binary {
        offset: 0x3f5f0,
        len: 0xfc,
        name: "tinyW7.exe",
    },
    Binary {
        offset: 0x3f6f0,
        len: 0x10c,
        name: "tinyW7x64.exe",
    },
    Binary {
        offset: 0x3f800,
        len: 0x10c,
        name: "tinyW7_3264.exe",
    },
    Binary {
        offset: 0x3f910,
        len: 0x61,
        name: "tinyXP.exe",
    },
    Binary {
        offset: 0x3f980,
        len: 0x400,
        name: "tls.exe",
    },
    Binary {
        offset: 0x3fd80,
        len: 0x400,
        name: "tls64.exe",
    },
    Binary {
        offset: 0x40180,
        len: 0x400,
        name: "tls_aoi.exe",
    },
    Binary {
        offset: 0x40580,
        len: 0x400,
        name: "tls_aoiOSDET.exe",
    },
    Binary {
        offset: 0x40980,
        len: 0x400,
        name: "tls_exiting.exe",
    },
    Binary {
        offset: 0x40d80,
        len: 0x400,
        name: "tls_import.exe",
    },
    Binary {
        offset: 0x41180,
        len: 0x400,
        name: "tls_k32.exe",
    },
    Binary {
        offset: 0x41580,
        len: 0x400,
        name: "tls_noEP.exe",
    },
    Binary {
        offset: 0x41980,
        len: 0x400,
        name: "tls_obfuscation.exe",
    },
    Binary {
        offset: 0x41d80,
        len: 0x400,
        name: "tls_onthefly.exe",
    },
    Binary {
        offset: 0x42180,
        len: 0x400,
        name: "tls_reloc.exe",
    },
    Binary {
        offset: 0x42580,
        len: 0x400,
        name: "tls_virtEP.exe",
    },
    Binary {
        offset: 0x42980,
        len: 0x41b,
        name: "truncatedlast.exe",
    },
    Binary {
        offset: 0x42da0,
        len: 0x400,
        name: "truncsectbl.exe",
    },
    Binary {
        offset: 0x431a0,
        len: 0x400,
        name: "version_cust.exe",
    },
    Binary {
        offset: 0x435a0,
        len: 0x400,
        name: "version_mini.exe",
    },
    Binary {
        offset: 0x439a0,
        len: 0x800,
        name: "version_std.exe",
    },
    Binary {
        offset: 0x441a0,
        len: 0x400,
        name: "virtEP.exe",
    },
    Binary {
        offset: 0x445a0,
        len: 0x600,
        name: "virtgap.exe",
    },
    Binary {
        offset: 0x44ba0,
        len: 0x25c,
        name: "virtrelocXP.exe",
    },
    Binary {
        offset: 0x44e00,
        len: 0x248,
        name: "virtsectblXP.exe",
    },
    Binary {
        offset: 0x45050,
        len: 0x1218,
        name: "weirdsord.exe",
    },
    Binary {
        offset: 0x46270,
        len: 0x400,
        name: "winver.exe",
    },
];
