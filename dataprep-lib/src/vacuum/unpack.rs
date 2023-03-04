use age::Decryptor;
use anyhow::{anyhow, Ok, Result};
use printio as _;
use std::{fs::File, io::BufReader, iter, path::PathBuf};
use std::path::Path;

use crate::types::unpack_plan::{UnpackPipelinePlan, UnpackPlan, UnpackType};

// Unpack a single file, directory, or symlink
pub async fn do_file_pipeline(
    UnpackPipelinePlan {
        origin_data,
        data_processing,
    }: UnpackPipelinePlan,
    input_dir: PathBuf,
    output_dir: PathBuf,
) -> Result<()> {
    // Processing directives require different handling
    match data_processing {
        UnpackType::File(UnpackPlan {
            compression,
            partition,
            encryption,
            writeout,
        }) => {
            // TODO (laudiacay) make sure you don't unpack things 23910 times with all the duplicates
            // TODO (organizedgrime) FILESYSTEM LOCKS ?

            // Construct the output path
            let output_path = output_dir.join(origin_data.original_location);

            // TODO (organizedgrime) async these reads. also is this buf setup right

            // If the file already exists, skip it- we've already processed it
            if Path::exists(&output_path) {
                // TODO make this a warning
                println!("File already exists: {}", output_path.display());
                return Ok(());
            }
            // otherwise make it
            let new_file_writer = File::create(output_path)?;

            // Ensure that our compression scheme is congruent with expectations
            // TODO use fancy .get_decoder() method :3
            assert_eq!(compression.compression_info, "ZSTD");

            // Create a new file writer
            // let mut new_file_writer = ZstdDecoder::new(new_file_writer).unwrap();

            // TODO (organizedgrime): switch back to iterating over chunks if use case arises
            // If there are chunks in the partition to process
            for chunk in writeout.chunk_locations.iter() {
                // Ensure that there is only one chunk
                // assert_eq!(partition.num_chunks, 1);
                // Chunk is a constant for now

                // Finish constructing the old file reader
                let old_file_reader = BufReader::new(File::open(chunk)?);

                // TODO naughty clone
                // Construct the old file reader by decrypting the encrypted piece
                let old_file_reader = {
                    // Match decryptor type to ensure compatibility;
                    // use internal variable to construct the decryptor
                    let decryptor = match Decryptor::new(old_file_reader)? {
                        Decryptor::Recipients(decryptor) => decryptor,
                        Decryptor::Passphrase(_) => {
                            return Err(anyhow!("Passphrase decryption not supported"))
                        }
                    };

                    // Use the decryptor to decrypt the encrypted piece; return result
                    decryptor.decrypt(iter::once(
                        &encryption.identity.clone() as &dyn age::Identity
                    ))?
                };

                // Copy the contents of the old reader into the new writer
                zstd::stream::copy_decode(old_file_reader, &new_file_writer).unwrap();

                // TODO check the encryption tag at the end of the file?
            }
            // Return OK status
            Ok(())
        }
        UnpackType::Directory => {
            // TODO naughty clone
            let loc = output_dir.join(origin_data.original_location.clone());
            // TODO (laudiacay) set all the permissions and stuff right?
            tokio::fs::create_dir_all(&loc).await.map_err(|e| e.into())
        }
        UnpackType::Symlink(to) => {
            // TODO naughty clone
            let loc = output_dir.join(origin_data.original_location.clone());
            // TODO (laudiacay) set all the permissions and stuff right?
            tokio::fs::symlink(loc, to).await.map_err(|e| e.into())
        }
    }
}

// TODO (thea-exe): Our inline tests
// Note (amiller68): Testing may rely on decrypting the file, which is not yet implemented
#[cfg(test)]
mod test {}
