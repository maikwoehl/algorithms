#!/bin/env python3
# -*- coding: utf-8 -*-

def main():
    elements = [3, 8, 4, 5, 9, 1, 7, 2, 6]

    print(elements)

    asc_elements = sort_ascending(elements)
    print(asc_elements)

    desc_elements = sort_descending(elements)
    print(desc_elements)

def sort_ascending(elements_array):
    return_array = elements_array
    swapped = True

    while swapped:
        for idx, item in enumerate(return_array):
            swapped = False
            if idx < len(return_array)-1:
                if return_array[idx] > return_array[idx+1]:
                    return_array[idx] = return_array[idx+1]
                    return_array[idx+1] = item
                    swapped = True
                    break

    return return_array

def sort_descending(elements_array):
    return_array = elements_array
    swapped = True

    while swapped:
        for idx, item in enumerate(return_array):
            swapped = False
            if idx < len(return_array)-1:
                if return_array[idx] < return_array[idx+1]:
                    return_array[idx] = return_array[idx+1]
                    return_array[idx+1] = item
                    swapped = True
                    break

    return return_array

if __name__ == "__main__":
    main()
