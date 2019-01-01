# Task
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.



# Example of usage:

```
# cargo run
Usage examples:
* Add Sally to Engineering
* List department Engineering
* List company
* Exit
Please input your command:
Add Sally to Engineering
Please input your command:
Add Adam to Engineering
Please input your command:
Add Tim to Sales
Please input your command:
Add Oliver to Sales
Please input your command:
Add Dan to Engineering
Please input your command:
List company
Employees in department Sales:
* Oliver
* Tim
Employees in department Engineering:
* Adam
* Dan
* Sally
Please input your command:
Exit
```
