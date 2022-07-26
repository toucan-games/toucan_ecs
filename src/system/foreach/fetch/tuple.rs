use toucan_ecs_macro::fetch_tuple;

use crate::error::FetchError;

use super::*;

// `Fetch` implemented for tuples of size 12 and less
fetch_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
fetch_tuple!(A, B, C, D, E, F, G, H, I, J, K);
fetch_tuple!(A, B, C, D, E, F, G, H, I, J);
fetch_tuple!(A, B, C, D, E, F, G, H, I);
fetch_tuple!(A, B, C, D, E, F, G, H);
fetch_tuple!(A, B, C, D, E, F, G);
fetch_tuple!(A, B, C, D, E, F);
fetch_tuple!(A, B, C, D, E);
fetch_tuple!(A, B, C, D);
fetch_tuple!(A, B, C);
fetch_tuple!(A, B);
fetch_tuple!(A);
