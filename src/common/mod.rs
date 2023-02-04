pub fn split_vector_by_type<T>(tokens: Vec<T>, splitter: T) -> Vec<Vec<T>> 
    where T: Clone + PartialEq 
{
    let mut result = Vec::new();

    let mut current_group = Vec::new();

    for token in tokens {
        match token {
            ref token_ref if token_ref == &splitter => {
                result.push(current_group.clone());
                current_group.clear();
            },
            _ => {
                current_group.push(token);
            }
        }
    }

    if !current_group.is_empty() {
        result.push(current_group);
    }

    return result;
}