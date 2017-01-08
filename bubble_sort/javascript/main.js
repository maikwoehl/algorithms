(function() {
    elements = [3, 8, 4, 5, 9, 1, 7, 2, 6];
    console.log(elements);

    asc_elements = sort("asc", elements);
    console.log(asc_elements);

    desc_elements = sort("desc", elements);
    console.log(desc_elements);
})();

function sort(type, elements) {
    var swapped = true;
    var return_elements = elements;

    while (swapped) {
        swapped = false;
        for(i = 0; i < return_elements.length; i++) {
            switch (type) {
                case "asc":
                    if (return_elements[i] > return_elements[i+1]) {
                        let item = return_elements[i]
                        return_elements[i] = return_elements[i+1]
                        return_elements[i+1] = item;
                        swapped = true;
                    }
                    break;

                case "desc":
                if (return_elements[i] < return_elements[i+1]) {
                        let item = return_elements[i]
                        return_elements[i] = return_elements[i+1]
                        return_elements[i+1] = item;
                        swapped = true;
                    }
                    break;
            }
        }
    }

    return return_elements;
}