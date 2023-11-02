# Using penpot to design the application

I decided to use penpot to design the frontend portion of the application.
Penpot is an open-source browser-based design tool that can be hosted locally.

I used docker as I already had the engine installed on my WSL2 instance.

https://help.penpot.app/technical-guide/getting-started/#install-with-docker

```bash
# Starting the container
docker compose -p penpot -f docker-compose.yaml up -d

# Closing the container
docker compose -p penpot -f docker-compose.yaml down
```

## Design process

For the frontend I kept most of the elements from the prototype version with a always visible navbar.
The navigation bar is moved to the left side of the application as the vertical space is more valuable than horizontal.
Although for portrait mode we can move the navbar to the bottom of the screen.

For the design I used some [Penpot libraries](https://penpot.app/libraries-templates).
Namely, 'Penpot - Design System v2.0', 'Tailwind Kit' and 'Bootstrap Icons'

I started by designing the sidebar, this sidebar should have all the navigation options that might be needed.
Expanded the options have descriptions on what they do, but if user wants they can also collapse the bar to get more screen space.

The theme is not final colours are subject to change depending on what will look ok in the final version.

Outside of the navigation elements I also included rough drafts of what the improved statistics should look like. 
They take into consideration that a transaction can be either positive or negative balance.

## Results

The rendered designs are available in this folder in PDF format. [Application-Draft.pdf](./Application-Draft.pdf)

Also included is the .penpot file to view the designs, this does not include the libraries mentioned previously to reduce file size.
The file is found in this folder as [FinanceApp.penpot](./FinanceApp.penpot)

