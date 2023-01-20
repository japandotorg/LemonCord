name: Bug Report
description: Create a report to help us improve (Report broken or incorrect behaviour).
labels: 'Type: Bug'
body:
  - type: markdown
    attributes:
      value: >
        Thanks for taking the time to fill out a bug report.
        If you want real-time support consider joining our Discord at https://melonbot.io/support instead.

        Please note that this form is for bugs only!

  - type: input
    attributes:
      label: Summary
      description: A simple summary of your bug report.
    validations:
      required: true
  
  - type: textarea
    attributes:
      label: Steps To Reproduce
      description: Steps to reproduce the behavior.
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
      label: Platform And Versions
      description: >
        OS: [eg: Windows]
        Rustc: [Run `cargo --version --verbose` and paste the information below.]
        LemonCord: [Version info can be found in the cargo.toml file of your local copy of the repository.]

  - type: textarea
    attributes:
      label: Additional Context
      description: If there's anything else to say, please do so here.
