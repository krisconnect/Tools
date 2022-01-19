```
char	*ft_strcpy(char *dest, char *src)
{
	int iterator;

	iterator = 0;
	while (*(src + iterator) != 0)
	{
		*(dest + iterator) = *(src + iterator);
		iterator++;
	}
	*(dest + iterator) = 0;
	return (dest);
}
```