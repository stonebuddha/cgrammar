#include <stdarg.h>

int sum_ints(int count, ...)
{
    va_list ap;
    va_start(ap, count);
    int total = 0;
    for (int i = 0; i < count; i++)
    {
        total += va_arg(ap, int);
    }
    va_end(ap);
    return total;
}

double first_double(int count, ...)
{
    va_list ap;
    va_start(ap, count);
    double d = va_arg(ap, double);
    char *s = va_arg(ap, char *);
    va_end(ap);
    (void)s;
    return d;
}
