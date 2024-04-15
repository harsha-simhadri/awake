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

// Function to simulate calling C++ function getValue and invoking the callback
void spin_and_call_back(void (*callback)(void *), void *context_handle)
{
    printf(L"spin_and_call_back started.\n");

    float i = 0;
    float sum = 0;
    for (i = 0; i < 1000000000; i++)
    {
        sum += i++;
    }

    printf(L"spin_and_call_back computed sum of billion numbers to be: %f\n", sum);

    callback(context_handle);

    printf(L"spin_and_call_back finished callback and is at the end of function\n");
}
