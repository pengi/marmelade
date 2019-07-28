use block;

impl ExtDescriptor {
    #[allow(non_snake_case)] // This struct comes from old Mac structs
    fn from(rdr: &mut FileBlockSeqReader) -> ExtDescriptor {
        let xdrStABN = rdr.read_u16();
        let xdrNumABlks = rdr.read_i16();
        ExtDescriptor {
            xdrStABN,
            xdrNumABlks,
        }
    }
}

impl ExtDataRec {
    fn from(rdr: &mut FileBlockSeqReader) -> ExtDataRec {
        ExtDataRec([
            ExtDescriptor::from(rdr),
            ExtDescriptor::from(rdr),
            ExtDescriptor::from(rdr),
        ])
    }
}

impl HfsMDB {
    #[allow(non_snake_case)] // This struct comes from old Mac structs
    pub fn from(block: &FileBlock) -> io::Result<HfsMDB> {
        let mut rdr = FileBlockSeqReader::from(block, 0);

        let mdb = HfsMDB {
            drSigWord: rdr.read_i16(),
            drCrDate: rdr.read_i32(),
            drLsMod: rdr.read_i32(),
            drAtrb: rdr.read_i16(),
            drNmFls: rdr.read_i16(),
            drVBMSt: rdr.read_i16(),
            drAllocPtr: rdr.read_i16(),
            drNmAlBlks: rdr.read_u16(),
            drAlBlkSiz: rdr.read_i32(),
            drClpSiz: rdr.read_i32(),
            drAlBlSt: rdr.read_i16(),
            drNxtCNID: rdr.read_i32(),
            drFreeBks: rdr.read_u16(),
            drVN: rdr.read_pstr(27),
            drVolBkUp: rdr.read_i32(),
            drVSeqNum: rdr.read_i16(),
            drWrCnt: rdr.read_i32(),
            drXTClpSiz: rdr.read_i32(),
            drCTClpSiz: rdr.read_i32(),
            drNmRtDirs: rdr.read_i16(),
            drFilCnt: rdr.read_i32(),
            drDirCnt: rdr.read_i32(),
            drFndrInfo: [
                rdr.read_i32(),
                rdr.read_i32(),
                rdr.read_i32(),
                rdr.read_i32(),
                rdr.read_i32(),
                rdr.read_i32(),
                rdr.read_i32(),
                rdr.read_i32(),
            ],
            drVCSize: rdr.read_i16(),
            drVBMCSize: rdr.read_i16(),
            drCtlCSize: rdr.read_i16(),
            drXTFlSize: rdr.read_i32(),
            drXTExtRec: ExtDataRec::from(&mut rdr),
            drCTFlSize: rdr.read_i32(),
            drCTExtRec: ExtDataRec::from(&mut rdr),
        };

        if mdb.drSigWord != 0x4244 {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid drSigWord"));
        }

        if mdb.drAlBlkSiz % 512 != 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid drAlBlkSiz"));
        }

        Ok(mdb)
    }
}
