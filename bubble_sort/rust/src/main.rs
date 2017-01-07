// bubble sort
// Maik Wöhl

// import for stdout().flush().unwrap()
use std::io::{self, Write};

fn main() {
    let elements: [i32; 9]  = [3, 5, 9, 2, 8, 7, 6, 1, 4];

    let mut asc_elements = elements;
    sort(SortDirection::Ascending, &mut asc_elements);

    let mut desc_elements = elements;
    sort(SortDirection::Descending, &mut desc_elements);

    print_array("Original", &elements);
    print_array("Ascending", &asc_elements);
    print_array("Descending", &desc_elements);
}

fn print_array(title: &str, array: &[i32]) {
    println!("{}:", title);
    for item in array.iter() {
        print!("{} ", item);
    }
    io::stdout().flush().unwrap();
    println!();
}

#[derive(Debug)]
enum SortDirection {
    Ascending,
    Descending,
}

fn sort(sort_direction: SortDirection, array_elements: &mut [i32]) {
    let mut swapped = true;

    while swapped {
        swapped = false;
        for idx in 0..(array_elements.len()-1) {
            match sort_direction {
                SortDirection::Ascending => {
                    if array_elements[idx] > array_elements[idx+1] {
                        let temp_item = array_elements[idx];
                        array_elements[idx] = array_elements[idx+1];
                        array_elements[idx+1] = temp_item;
                        swapped = true;
                        break;
                    }
                },
                SortDirection::Descending => {
                    if array_elements[idx] < array_elements[idx+1] {
                        let temp_item = array_elements[idx];
                        array_elements[idx] = array_elements[idx+1];
                        array_elements[idx+1] = temp_item;
                        swapped = true;
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use super::SortDirection;
    #[test]
    fn test_ascending() {
        let mut elements: [i32; 4] = [9, 5, 3, 7];
        {
            sort(SortDirection::Ascending, &mut elements);
        }

        assert_eq!([3, 5, 7, 9], elements);
    }

    #[test]
    fn test_descending() {
        let mut elements: [i32; 4] = [3, 8, 4, 1];
        {
            sort(SortDirection::Descending, &mut elements)
        }

        assert_eq!([8, 4, 3, 1], elements);
    }
}
