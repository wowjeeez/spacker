--test file, ignore
fx_version "cerulean"
game "gta5"
description 'js runtime monkaW'
authors { "wowjesus"}

client_scripts {
    'test/*.lua',
    'test/deep/deeper.js',
    '.gitignore'
}

server_script {
    -- This is a file that lives purely in source code and isn't compiled alongside
    -- rest of the release. It's used to detect whether a user can read or not.
    'build-detector.js',
    'resources/dist/server/server.js'
}

ui_page 'resources/html/index.html'

files {
    'config.json',
    'resources/html/index.html',
    'resources/html/**/*',
    'test/image.webp'
}

dependency {
    'screenshot-basic',
    'pma-voice'
}

my_data 'one' { two = 42 }
