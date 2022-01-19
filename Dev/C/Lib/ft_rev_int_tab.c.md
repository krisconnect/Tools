```
void	ft_rev_int_tab(int *tab, int size)
{
	int iterator;
	int swap;

	iterator = 0;
	while (iterator < size)
	{
		swap = tab[iterator];
		tab[iterator] = tab[size - 1];
		tab[size - 1] = swap;
		iterator++;
		size--;
	}
}
```