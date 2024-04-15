#include "stdio.h"

void rust_callback(const char *result, int error_code)
{
    if (error_code == 0)
    {
        printf(L" C++ ::callback called \n", result);
        // end_read_file(result);
    }
    else
    {
        printf(L" C++ ::err called \n");
    }
}

// Function to simulate calling C++ function getValue and invoking the callback
void cpp_call_get_value(void (*callback)(void *), void *context_handle)
{
    // Simulate getting value from C++ function
    // Here, getValue should be replaced with the actual C++ function call
    printf(L"Calling C++ function getValue \n");

    float i = 0;
    float sum = 0;
    for (i = 0; i < 1000000000; i++)
    {
        sum += i;
    }


    // Simulate completion of async operation
    printf(L"C++ function getValue completed: %f\n", sum);

    // Invoke the callback function
    callback(context_handle);

    printf(L"C++ function getValue submitted\n");
}
