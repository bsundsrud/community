# community

Community helps find social services and programs that a user qualifies for.

## Roadmap
- [ ] Better name

- [ ] Constraint-checking system that matches a user's info against stored organizations/programs
    - [X] Rudimentary rule-checking support
    - [X] Can handle complex requirement schemes
        - [X] OR
        - [X] AND
        - [X] nested OR
        - [X] nested AND
    - [ ] Persistent storage for organizations, programs, checklists, and requirements
    - [ ] Location-based searching
    - [ ] category-based searching
- [ ] Web
    - [ ] Infrastructure
        - [ ] REST API using iron/nickel/rustful/something else fun
        - [ ] Mobile-friendly (both UX and page size/load times)
        - [ ] Progressive enhancement, navigable without JavaScript enabled
    - [ ] Content
        - [ ] 'Find available services' using above constraint system
        - [ ] General browsing
        - [ ] Searching based on keywords, service categories, other tags
        - [ ] ability for users to submit information on a new/existing service, which will go into a review queue
        - [ ] category-aware extra information (i.e., selecting a category like
            'Heating assistance' can display relevant state law about when heat
            can or can't be shut off)
