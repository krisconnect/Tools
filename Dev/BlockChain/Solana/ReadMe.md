# My collection of useful stuff: Solana
### Tests
### Stuff to remember
##### spl-token-cli
0. Have the ```solana config get``` return that you are connected to the right url and have a distinct account.
1. Generate a fungible token
```bash
❯ spl-token create-token
Creating token Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA


Address:  Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP
Decimals:  9

Signature: 2gdZ86gqy6rnMBsApZg3aMpAVu7eR2HpEnH3DjcniccHHkFrHtEGJixPTnWUvhaBgnvkF2HR7MzE6q1GcUFbPMp8

```
Where the token's name is ```Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP``` 

2. Check supply with the token name/address, which should be zero.

```bash
❯ spl-token supply Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP
0
```
3. Create an account to hold the token

```bash
❯ spl-token create-account Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP
Creating account 8XFm39uep9qwk2oNu1zVMtFjaFLP4TL33Q1R7BPMYXxY

Signature: 61SRAgFQ39QC38a8BP7WprtNsUGCv75kgCE9pvpKKrCNa1GQ1VTwo9362G9KkHMkMjWzDe6pvV9UqdHgZcA3xNqb
```
The token name is still ```Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP```
and the token holder accounts address is ```8XFm39uep9qwk2oNu1zVMtFjaFLP4TL33Q1R7BPMYXxY```
4. Mint a bunch of them tokens!

```bash
❯ spl-token mint Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP 1000
Minting 1000 tokens
  Token: Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP
  Recipient: 8XFm39uep9qwk2oNu1zVMtFjaFLP4TL33Q1R7BPMYXxY

Signature: 3Gp9yuDuVQqXLvZjL1vxAUfQgQ3EDwj1FJsACAPXwTA9Y7jRBKWj1km8CVNu6idnEMpYuZSGeqwjTzJLMzTVHrwc
```
This mints the tokens to the token account.

5. Verify:
```bash
❯ spl-token accounts
Token                                         Balance
-----------------------------------------------------
Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP  1000
```
At this point we have our own account, we have an account that mints tokens and we need another account to send our tokens to. The recieving account must also create a token account for the same token then the transaction can be completed.

6. Send **[token]** **[amount]** **[to address]**
```bash
❯ spl-token transfer Bd43VaCYETbV6CqMThHUQe39fniKUGGBTe53AuxVpgsP 500 8t4V9aYB27oeu5VNBirjAUohg1M3GZgcZ5NcQLti5Z2i
Transfer 500 tokens
  Sender: 8XFm39uep9qwk2oNu1zVMtFjaFLP4TL33Q1R7BPMYXxY
  Recipient: 8t4V9aYB27oeu5VNBirjAUohg1M3GZgcZ5NcQLti5Z2i
  Recipient associated token account: 9ZguDVov38pB913DPXuUHgaNps3BPCVY9Qn1sMzhn8he

Signature: hg4MYAuhvvmBE3CxJgLVPr2dgTaGn1Mzff6THcUjmPWfNaJy431sks2BCaHcsVSxJhyhp3p5NjVjVPBQUKsdsN5
```