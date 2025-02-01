tantalos = Tantalos
info = About
settings = Settings
accounts = Accounts
language = { $selector ->
    [true] Language:
   *[false] {""}
} {$lang ->
    *[default] Unknown
    [en-US] English
    [fi-FI] Finnish (Suomen kieli)
}
menu-side = {$selector ->
    [true] Menu location:
   *[false] {""}
} {$side ->
    *[default] unknown
    [Left] left
    [Right] right
    [Up] above
    [Down] below
}
logged-in-accounts-count = { $count ->
   [0] You aren't logged in to any accounts currently
   [1] You are currently logged in to only one account
  *[other] You are currently logged in to {$count} accounts
}
