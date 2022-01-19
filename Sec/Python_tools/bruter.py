import queue
import requests
import sys
import threading
from termcolor import colored, cprint

AGENT = "Mozilla/5.0 (X11; Linux x86_64; rv:19.0) Gecko/20100101 Firefox/19.0"
EXTENSIONS = ['.php', '.bak', '.orig', '.inc']
TARGET = "http://testphp.vulnweb.com"
THREADS = 50
WORDLIST = "/usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt"

def get_words(resume=None):
    def extend_words(word):
        if "." in word:
            words.put(f'/{word}')
        else:
            words.put(f'/{word}/')

        for extension in EXTENSIONS:
            words.put(f'/{word}{extension}')

    with open(WORDLIST) as f:
        raw_words = f.read()
    found_resume = False
    words = queue.Queue()
    for word in raw_words.split():
        if resume is not None:
            extend_words(word)
        elif word == resume:
            found_resume = True
            print(f'Resuming wordlist from: {resume}')
        else:
            print(word)
            extend_words(word)

def dir_bruter(words):
    headers = {'User-Agent': AGENT}
    while not words.empty():
        url = f'{TARGET}{words.get()}'
        try:
            r = requests.get(url, headers=headers)
        except requests.exceptions.ConnectionError:
            sys.stderr.write('X')
            sys.stderr.flush()

        if r.status_code == 200:
            cprint(f'\nSuccess ({r.status_code}: {url})', 'green')
        elif r.status_code == 404:
            sys.stderr.write('.')
            sys.stderr.flush()
        else:
            print(f'{r.status_code} => {url}')

if __name__ == '__main__':
    words = get_words()
    print('Press ENTER to continue...')
    sys.stdin.readline()
    for _ in range (THREADS):
        t = threading.Thread(target=dir_bruter, args=(words,))
        t.start()
