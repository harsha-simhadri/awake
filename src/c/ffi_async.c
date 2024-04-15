#include "stdio.h"

// Spin for a while and then call the callback
void spin_and_call_back(void (*callback)(void *), void *context_handle)
{
    long long unsigned int i = 0;
    long long unsigned int sum = 0;
    for (i = 0; i < 1000000000; i++)
    {
        sum += i++;
    }
    callback(context_handle);
}

// Sum billion numbers and return the result and callback
void add_to_billion_and_call_back(long long unsigned int* sum, void (*callback)(void *),  void *context_handle)
{
    long long unsigned int i = 0;
    *sum = 0;
    for (i = 0; i < 1000000000llu; i++)
    {
        *sum += i;
    }
    callback(context_handle, sum);
}