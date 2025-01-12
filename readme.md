# Tabby-note

Quickly write and saves notes in their respective place.

## Commands

### t / title
Prepend a title, this can also be used for the filename.

### w / write
Append a string and save as a file to configured location.

### r / retry
This will reopen the written file for ammending.

### config
This will create a config file in your config directory: `./config/tn`.


## Templating
The templating engine uses minijinja - [Datetime Format](https://docs.rs/minijinja-contrib/latest/minijinja_contrib/filters/fn.datetimeformat.html).

Refer to `default-template.jinja`, this is not automatically created.

### Keys
The keys avalible within a template.
#### title
`{{ title }}`

#### content
`{{ content }}`

#### now
`{{ now|datetimeformat(format="short") }}`
