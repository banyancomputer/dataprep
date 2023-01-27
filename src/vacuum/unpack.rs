use anyhow::Result;
use tokio::fs::File;

use crate::crypto_tools::decryption_reader::DecryptionReader;
use crate::types::pipeline::{DataProcess, PipelineToDisk};
use crate::types::shared::DataProcessDirectiveToDisk;
use flate2::read::GzDecoder;
use std::path::PathBuf;
use crate::types::plan::DataProcessPlan;

pub(crate) async fn do_file_pipeline(
    PipelineToDisk {
        origin_data,
        data_processing,
    }: PipelineToDisk,
    output_root: PathBuf,
) -> Result<()> {
    match data_processing {
        DataProcessDirectiveToDisk::File(DataProcess {
            compression,
            partition,
            encryption,
            writeout,
        }) => {
            // TODO (laudiacay) async these reads. also is this buf setup right

            let mut new_file_writer =
                std::fs::File::create(output_root.join(origin_data.original_location)).await?;

            for chunk in 0..partition.0.num_chunks {
                // open a reader to the original file
                let old_file_reader = tokio::io::BufReader::new(tokio::fs::File::open(
                    &writeout.chunk_locations.get(chunk),
                )?);
                // put a gzip encoder on it then buffer it
                assert_eq!(compression.compression_info, "GZIP");
                let mut old_file_reader = std::io::BufReader::new(GzDecoder::new(old_file_reader));
                let mut old_file_reader =
                    DecryptionReader::new(old_file_reader, encryption.encrypted_pieces.get(i).key).await;

                std::io::copy(&mut old_file_reader, &mut new_file_writer)?;
                // TODO check the encryption tag at the end of the file
                // old_file_reader.finish()?;
            }
            Ok(())
        }
        DataProcessDirectiveToDisk::Directory => {
            let loc = output_root.join(origin_data.original_location);
            // TODO (laudiacay) set all the permissions and stuff right?
            tokio::fs::create_dir_all(&loc).await.map_err(|e| e.into())
        }
        DataProcessDirectiveToDisk::Symlink => {
            let loc = output_root.join(origin_data.original_location);
            // TODO (laudiacay) set all the permissions and stuff right?
            tokio::fs::create_dir_all(&loc).await.map_err(|e| e.into())
        }
        DataProcessDirectiveToDisk::Duplicate(_smtd) => {
            todo!("hold off on duplicates for now");
        }
    }
}
// TODO (xBalbinus & thea-exe): Our inline tests
// Note (amiller68): Testing may rely on decrypting the file, which is not yet implemented
#[cfg(test)]
mod test {
    #[test]
    fn test() {
        todo!("Test compression and encryption");
    }
}
