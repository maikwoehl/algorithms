/*
 * main.cpp
 *
 *  Created on: 02.04.2017
 *      Author: maik
 */

#include <bitset>
#include <iostream>
#include <string>
#include <sstream>

int int_to_bcd(int x);

int main()
{
	int x = 12;

	int x_bcd = int_to_bcd(x);

	std::stringstream ss;

	ss << std::bitset<4>(x_bcd>>4).to_string().c_str() << " " << std::bitset<4>(x_bcd).to_string().c_str();

	puts(ss.str().c_str());

	return 0;
}

int int_to_bcd(int x)
{
	int power_of_ten = 10;
	int digits = 2;
	while ((x / power_of_ten) >= 1)
	{
		power_of_ten = power_of_ten * 10;
		digits = digits + 1;
	}

	// One power greater than needed, so decrease it
	power_of_ten = power_of_ten / 10;
	digits = digits -2;

	int bcd_number = 0;
	int number = x;

	while (digits >= 0)
	{
		int div_result = int((number / power_of_ten));
		// Add n-th digit to bcd_number and shift it to n-th bcd nibble
		bcd_number = bcd_number + (div_result << digits*4);
		// Decrease number by the calculated power_of_ten (e.g. 312 - 300 = 12)
		number = number - div_result * power_of_ten;
		power_of_ten = power_of_ten / 10;
		digits = digits - 1;
	}

	return bcd_number;
}


