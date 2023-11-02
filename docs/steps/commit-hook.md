# Git commit hook

To validate our application before it is pushed to the remote repository we should run all the steps required by the build.

1. Verify the code is correctly formatted (prettier)
2. Check that some basic rules are met (eslint & cargo check)
3. Test the code (cargo test & npm run vitest run)

For this we can create a simple script that will call our `npm` scripts.
You should read the docs for [installing testing](./installing-testing.md) before this file.

```bash
#!/bin/bash

# Verify the formatting and linting are followed
npm run lint

# Make quick verification of the rust project
npm run cargo check

# Runs both test:frontend & test:backend
npm run test

exit 0
```

This file is found in [scripts/pre-commit](../../scripts/pre-commit)  
Note the file should be converted into a executable with the command `chmod +x pre-commit`  
This file should be copied into the `.git/hooks/` folder.  
This can be done with the following command in root of project `cp scripts/pre-commit .git/hooks/`  


