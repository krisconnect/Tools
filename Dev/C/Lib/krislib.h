#include <stdlib.h>
#include <unistd.h>

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



void	ft_putchar(char c)
{
	write(1, &c, 1);
}



void	ft_print_hexa(char c)
{
	int		negative_char;
	char	hex_n1;
	char	hex_n2;

	if (c < 0)
	{
		negative_char = (-128 - (c)) * -1 + 128;
		hex_n1 = negative_char / 16;
		hex_n2 = negative_char % 16;
	}
	else
	{
		hex_n1 = c / 16;
		hex_n2 = c % 16;
	}
	if (hex_n1 < 10)
		ft_putchar(hex_n1 + '0');
	else
		ft_putchar(hex_n1 + 87);
	if (hex_n2 < 10)
		ft_putchar(hex_n2 + '0');
	else
		ft_putchar(hex_n2 + 87);
}



void	ft_putaddr(void *addr)
{
	char	addr_c[15];
	long	addr_l;
	int		i;

	addr_l = (long)addr;
	i = 0;
	while (addr_l > 0)
	{
		addr_c[i] = addr_l % 16;
		if (addr_c[i] < 10)
			addr_c[i] += '0';
		else
			addr_c[i] += 87;
		addr_l = addr_l / 16;
		i++;
	}
	while (i < 15)
	{
		addr_c[i] = '0';
		i++;
	}
	while (--i >= 0)
		ft_putchar(addr_c[i]);
}



void	ft_print_line(void *addr, int size)
{
	int		i;
	int		sp_to_print;
	char	*addr_c;

	addr_c = (char *)addr;
	i = 0;
	while (i < size)
	{
		ft_print_hexa(addr_c[i++]);
		if (i % 2 == 0)
			ft_putchar(' ');
	}
	sp_to_print = ((16 - size) * 2) + (19 - size) / 2;
	while (--sp_to_print > 0)
		ft_putchar(' ');
	i = -1;
	while (++i < size)
	{
		if (addr_c[i] >= 32 && addr_c[i] <= 126)
			ft_putchar(addr_c[i]);
		else
			ft_putchar('.');
	}
}



void	*ft_print_memory(void *addr, unsigned int size)
{
	unsigned int	char_to_print;
	unsigned int	offset;

	offset = 0;
	while (size > 0)
	{
		if (size >= 16)
		{
			char_to_print = 16;
			size -= 16;
		}
		else
		{
			char_to_print = size;
			size = 0;
		}
		ft_putaddr(addr + offset);
		ft_putchar(':');
		ft_putchar(' ');
		ft_print_line(addr + offset, char_to_print);
		ft_putchar('\n');
		offset += 16;
	}
	return (addr);
}



void	ft_putnbr(int nb)
{
	unsigned int nbr_unsigned;

	if (nb < 0)
	{
		nbr_unsigned = (unsigned int)(-1 * nb);
		ft_putchar('-');
	}
	else
		nbr_unsigned = (unsigned int)nb;
	if (nbr_unsigned >= 10)
	{
		ft_putnbr(nbr_unsigned / 10);
		ft_putnbr(nbr_unsigned % 10);
	}
	else
		ft_putchar(nbr_unsigned + '0');
}



void	ft_putstr(char *str)
{
	char *charptr;

	charptr = str;
	while (*charptr != 0)
	{
		write(1, charptr, 1);
		charptr++;
	}
}



int	*ft_range(int min, int max)
{
	int *tab;
	int i;

	i = 0;
	if (min >= max)
		return (0);
	tab = (int *)malloc(sizeof(int) * (max - min));
	while (min < max)
	{
		*(tab + i) = min;
		min++;
		i++;
	}
	return (tab);
}



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



void	ft_sort_int_tab(int *tab, int size)
{
	int i;
	int j;
	int temp;

	i = 0;
	while (i < size)
	{
		j = i + 1;
		while (j < size)
		{
			if (*(tab + j) < *(tab + i))
			{
				temp = *(tab + i);
				*(tab + i) = *(tab + j);
				*(tab + j) = temp;
			}
			j++;
		}
		i++;
	}
}



char	*ft_strncpy(char *dest, char *src, unsigned int n)
{
	unsigned int		i;

	i = 0;
	while (src[i] && i < n)
	{
		dest[i] = src[i];
		i++;
	}
	while (i < n)
	{
		dest[i] = '\0';
		i++;
	}
	dest[i] = '\0';
	return (dest);
}



int		is_in_charset(char *charset, char to_find)
{
	int i;

	i = 0;
	while (charset[i])
	{
		if (to_find == charset[i])
			return (1);
		i++;
	}
	return (0);
}



char	*ft_get_next_str(char **pos_in_str, char *charset, int *next_str_len)
{
	int		i;
	char	*str_start;

	*next_str_len = 0;
	str_start = 0;
	i = 0;
	while ((*pos_in_str)[i])
	{
		if (is_in_charset(charset, (*pos_in_str)[i]) && str_start != 0)
		{
			*pos_in_str = str_start + *next_str_len;
			return (str_start);
		}
		else if (!is_in_charset(charset, (*pos_in_str)[i]) && str_start == 0)
			str_start = &(*pos_in_str)[i];
		if (!is_in_charset(charset, (*pos_in_str)[i]))
			*next_str_len = *next_str_len + 1;
		i++;
	}
	*pos_in_str = str_start + *next_str_len;
	if (*next_str_len == 0)
		return (0);
	return (str_start);
}



char	**ft_build_tab(char *str, char *charset)
{
	int		nb_str;
	char	**strs;
	int		next_str_len;
	char	*pos_in_str;

	nb_str = 0;
	next_str_len = 0;
	pos_in_str = str;
	while (ft_get_next_str(&pos_in_str, charset, &next_str_len))
		nb_str++;
	if (!(strs = (char **)malloc(sizeof(char *) * (nb_str + 1))))
		return (0);
	return (strs);
}



char	**ft_split(char *str, char *charset)
{
	char	**strs;
	int		next_str_len;
	char	*next_str;
	char	*pos_in_str;
	int		i;

	if (!(strs = ft_build_tab(str, charset)))
		return (0);
	i = 0;
	pos_in_str = str;
	while ((next_str = ft_get_next_str(&pos_in_str, charset, &next_str_len)))
	{
		if (!(strs[i] = (char *)malloc(sizeof(char) * next_str_len + 1)))
			return (0);
		ft_strncpy(strs[i], next_str, next_str_len);
		i++;
	}
	strs[i] = 0;
	return (strs);
}



int		an(char c)
{
	if (c < '0')
		return (0);
	else if (c > '9' && c < 'A')
		return (0);
	else if ((c > 'Z' && c < 'a') || (c > 'z'))
		return (0);
	return (1);
}



char	*ft_strcapitalize(char *str)
{
	char *charptr;

	charptr = str;
	while (*charptr)
	{
		if (charptr == str)
		{
			if (*charptr <= 'z' && *charptr >= 'a')
				*charptr -= 32;
		}
		else if (an(*(charptr - 1)) == 0)
		{
			if (*charptr <= 'z' && *charptr >= 'a')
				*charptr -= 32;
		}
		else if ((*charptr >= 'A') && (*charptr <= 'Z'))
		{
			*charptr += 32;
		}
		charptr++;
	}
	return (str);
}



char	*ft_strcat(char *dest, char *src)
{
	char *charptr;
	char *charptrsrc;

	charptr = dest;
	charptrsrc = src;
	while (*charptr)
		charptr++;
	while (*charptrsrc)
	{
		*charptr = *charptrsrc;
		charptrsrc++;
		charptr++;
	}
	*charptr = 0;
	return (dest);
}



int	ft_strcmp(char *s1, char *s2)
{
	while (*s1 && *s2 && (*s1 == *s2))
	{
		s1++;
		s2++;
	}
	return (*s1 - *s2);
}



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



int		ft_strlen(char *str)
{
	char	*charptr;
	int		iterator;

	charptr = str;
	iterator = 0;
	while (*charptr != 0)
	{
		iterator++;
		charptr++;
	}
	return (iterator);
}



char	*ft_strdup(char *src)
{
	char	*dupli;
	int		i;

	i = 0;
	if (!(dupli = malloc(ft_strlen(src) + 1)))
		return (dupli);
	while (src[i])
	{
		dupli[i] = src[i];
		i++;
	}
	dupli[i] = 0;
	return (dupli);
}



void	printsep(char **concatptr, char *sep)
{
	while (*sep)
	{
		**concatptr = *sep;
		sep++;
		*concatptr += 1;
	}
}



void	makeconcat(int size, char *concatptr, char **strs, char *sep)
{
	int n;
	int i;

	n = 0;
	while (n < size)
	{
		i = 0;
		while (strs[n][i])
		{
			*concatptr = strs[n][i];
			concatptr++;
			i++;
		}
		if (n == size - 1)
			*concatptr = 0;
		else if (*sep)
			printsep(&concatptr, sep);
		n++;
	}
}



char	*ft_strjoin(int size, char **strs, char *sep)
{
	char	*concat;
	char	*concatptr;
	int		nbchar;

	if (size)
	{
		nbchar = (strs[size - 1] - *strs) + ft_strlen(strs[size - 1]) + 1;
		concat = (char *)malloc(nbchar + ((ft_strlen(sep) - 1) * (size - 1)));
	}
	else
	{
		concat = malloc(1);
		*concat = 0;
		return (concat);
	}
	concatptr = concat;
	makeconcat(size, concatptr, strs, sep);
	return (concat);
}



unsigned int	ft_strlcat(char *dst, char *src, unsigned int siz)
{
	char			*d;
	char			*s;
	unsigned int	n;
	unsigned int	dlen;

	d = dst;
	s = src;
	n = siz;
	while (n-- != 0 && *d != '\0')
		d++;
	dlen = d - dst;
	n = siz - dlen;
	if (n == 0)
		return (dlen + ft_strlen(s));
	while (*s)
	{
		if (n != 1)
		{
			*d++ = *s;
			n--;
		}
		s++;
	}
	*d = '\0';
	return (dlen + (s - src));
}



char	*ft_strncat(char *dest, char *src, unsigned int nb)
{
	char			*charptrdest;
	char			*charptrsrc;
	unsigned int	i;

	i = 0;
	charptrdest = dest;
	charptrsrc = src;
	while (*charptrdest)
		charptrdest++;
	while (i < nb && *charptrsrc)
	{
		*charptrdest = *charptrsrc;
		charptrsrc++;
		charptrdest++;
		i++;
	}
	*charptrdest = 0;
	return (dest);
}



int	ft_strncmp(char *s1, char *s2, unsigned int n)
{
	if (n == 0)
		return (0);
	while (n != 0)
	{
		if (*s1 != *s2++)
			return (*(unsigned char *)s1 - *(unsigned char *)(s2 - 1));
		if (*s1++ == '\0')
			break ;
		n--;
	}
	return (0);
}



char	*ft_strstr(char *str, char *to_find)
{
	int i;
	int j;

	if (*to_find == 0)
		return (str);
	i = 0;
	while (str[i] != '\0')
	{
		j = 0;
		while (to_find[j] == str[i + j])
		{
			if (to_find[j + 1] == '\0')
			{
				return (str + i);
			}
			j++;
		}
		i++;
	}
	return (0);
}
