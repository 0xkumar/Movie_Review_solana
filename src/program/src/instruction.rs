use borsh::{BorshDeserialize};
use solana_program::program_error::ProgramError;

pub enum MovieInstruction{
    AddMovieReview{
        title:String,
        rating: u8,
        description: String
    },

    UpdateMovieReview{
        title:String,
        rating: u8,
        description: String
    },

    AddComment{
        comment:String,
    },
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload{
    title: String,
    rating: u8,
    description:String
}


#[derive(BorshDeserialize)]
struct commentPayload{
    comment: String,
}

//Implementing the Enum
impl MovieInstruction{
    //Receives the 'instruction data in bytes'
    pub fn unpack(input: &[u8]) -> Result <Self, ProgramError>{
        //'split_first' splits the total bytes into 2 parts 1. first bytes 2. Remaining all bytes
        //'ok_or' converts the option to a Result.
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        //'try_from_slice' deserialises the bytes to the type defined before. Here it desearialises the 
        // bytes to 'MovieReviewPayload' struct and returns the payload instance.
        //let payload = MovieReviewPayload::try_from_slice(rest).map_err(|_| ProgramError::InvalidInstructionData)?;

        match variant{
            0 =>{
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Ok(Self::AddMovieReview{
                    title: payload.title,
                    rating: payload.rating,
                    description: payload.description,
                })
            },
            
            1 =>{
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Ok(Self::UpdateMovieReview{
                    title: payload.title,
                    rating: payload.rating,
                    description: payload.description,
                })
            },

            2 => {
                let payload = commentPayload::try_from_slice(rest).unwrap();
                Ok(Self::AddComment{
                    comment: payload.comment,
                })
            },
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
