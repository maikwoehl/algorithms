(function() {
    elements = [3, 8, 4, 5, 9, 1, 7, 2, 6];
    console.log(elements);

    asc_elements = sort(SortDirection.Ascending, elements);
    console.log(asc_elements);

    desc_elements = sort(SortDirection.Descending, elements);
    console.log(desc_elements);
})();

var SortDirection = {
    Ascending: 1,
    Descending: 2
};

function sort(type, elements) {
    var swapped = true;
    var return_elements = elements;

    while (swapped) {
        swapped = false;
        for(i = 0; i < return_elements.length; i++) {
            switch (type) {
                case SortDirection.Ascending:
                    if (return_elements[i] > return_elements[i+1]) {
                        let item = return_elements[i]
                        return_elements[i] = return_elements[i+1]
                        return_elements[i+1] = item;
                        swapped = true;
                    }
                    break;

                case SortDirection.Descending:
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