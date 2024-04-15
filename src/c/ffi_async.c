#include "stdio.h"

// void rust_callback(const char *result, int error_code)
// {
//     if (error_code == 0)
//     {
//         printf(L" C++ ::callback called \n", result);
//         // end_read_file(result);
//     }
//     else
//     {
//         printf(L" C++ ::err called \n");
//     }
// }

// Spin for a while and then call the callback
void spin_and_call_back(void (*callback)(void *), void *context_handle)
{
    printf(L"spin_and_call_back started.\n");

    long long unsigned int i = 0;
    long long unsigned int sum = 0;
    for (i = 0; i < 1000000000; i++)
    {
        sum += i++;
    }

    printf(L"spin_and_call_back computed sum of billion numbers to be: %llu\n", sum);

    callback(context_handle);

    printf(L"spin_and_call_back finished callback and is at the end of function\n");
}

// Sum billion numbers and return the result and callback
void add_to_billion_and_call_back(long long unsigned int* sum, void (*callback)(void *),  void *context_handle)
{
    printf(L"sum_billion_numbers_and_call_back started.\n");

    long long unsigned int i = 0;
    *sum = 0;
    for (i = 0; i < 1000000000llu; i++)
    {
        *sum += i;
    }

    printf(L"sum_billion_numbers_and_call_back computed sum of billion numbers to be: %llu\n", sum);

    callback(context_handle, sum);

    printf(L"sum_billion_numbers_and_call_back finished callback and is at the end of function\n");
}