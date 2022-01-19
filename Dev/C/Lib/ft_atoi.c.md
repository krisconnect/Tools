```
int	ft_atoi(char *str)
{
	int		number;
	int		sign;
	char	*charptr;

	number = 0;
	sign = 1;
	charptr = str;
	while (*charptr == ' ' || (*charptr >= 9 && *charptr <= 13))
		charptr++;
	while (*charptr == '-' || *charptr == '+')
	{
		if (*charptr == '-')
			sign *= -1;
		charptr++;
	}
	while (*charptr >= '0' && *charptr <= '9')
	{
		number *= 10;
		number += (int)(*charptr - '0');
		charptr++;
	}
	return (number * sign);
}
```