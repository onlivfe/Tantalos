tantalos = Tantalos

### Platform names
vrchat = VRChat
chilloutvr = ChilloutVR
resonite = Resonite
### /Platform names

### Page names
info = About
settings = Settings
accounts = Accounts
dashboard = Dashboard
### /Page names

### Info page
placeholder-text = Consequatur quas voluptate hic fugit doloremque impedit quam velit. Impedit sint est consequatur architecto rerum deleniti eaque omnis. Qui praesentium voluptate quibusdam dolor quae officiis perferendis. Quis illum repellat et adipisci ad voluptas. Occaecati velit id sint sint est blanditiis. Aut adipisci commodi repellendus sunt delectus architecto molestiae.
### /Info page

### Settings page
language = { $selector ->
    [true] Language:
   *[false] {""}
} {$lang ->
    *[default] Unknown
    [en-US] English
    [fi-FI] Finnish (Suomen kieli)
}
menu-side = { $selector ->
    [true] Menu location:
   *[false] {""}
} {$side ->
    *[default] unknown
    [Left] left
    [Right] right
    [Up] above
    [Down] below
}
### /Settings page

### Login related
logged-in-accounts-count = { $count ->
   [0] You aren't logged in to any accounts currently
   [1] You are currently logged in to only one account
  *[other] You are currently logged in to {$count} accounts
}
add-account = Add an account
email = Email
password = Password
totp = 2FA code
enable-totp = 2FA enabled
login = Login
### /Login related
