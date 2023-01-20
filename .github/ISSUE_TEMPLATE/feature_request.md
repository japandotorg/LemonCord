name: Feature Request
description: For feature requests regarding LemonCord itself.
lables: 'Type: Feature'
body:
  - type: markdown
  attributes:
    value: |
      Thank you for taking the time to fill our an issue, this template is meant for any feature suggestions.
      If you want real-time support consider joining our Discord at https://melonbot.io/support instead.

  - type: textarea
    attributes:
      label: Describe The Feature Request
      description: A clear and concise description of what the feature is.
    validations:
      required: true

  - type: textarea
    attributes:
      label: Expected Behavior
      description: A clear and concise description of what is expected to happen.
    validations:
      required: true

  - type: textarea
    attributes:
      label: Anything else?
      description: Let us know if you have anything else to share.
