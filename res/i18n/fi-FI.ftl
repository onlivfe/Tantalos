tantalos = Tantalos
info = Tietoja
settings = Asetukset
accounts = Käyttäjätilit
language = { $selector ->
    [true] Kieli:
   *[false] {""}
} {$lang ->
    *[default] Unknown
    [en-US] Englannin kieli (English)
    [fi-FI] Suomen kieli
}
menu-side = {$selector ->
    [true] Valikon sijainti:
   *[false] {""}
} {$side ->
    *[default] tuntematon
    [Left] vasemmalla
    [Right] oikealla
    [Up] yllä
    [Down] alla
}
logged-in-accounts-count = { $count ->
   [0] Et ole tällä hetkellä kirjautunut sisään yhteenkään palveluun
   [1] Olet tällä hetkellä kirjautunut sisään vain yhteen palveluun
  *[other] Olet tällä hetkellä kirjautunut sisään {$count} palveluun
}
