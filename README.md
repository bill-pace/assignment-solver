# assignment-solver
A min cost/max flow network solver for the assignment problem.

## Overview
This Rust application takes as input a set of workers and a set of tasks, then finds an optimal way to assign those workers to tasks. To run the application, launch the executable then select your input and output files with the dialogs, then click the run button. A progress bar will appear, and when finished the UI will report success or an error message in the bottom panel while allowing you to select another pair of input/output files.

For each task, the required inputs are the minimum number of workers needed to complete the task and the maximum number of workers that could be assigned to the task. For example, if you are operating a warehouse that needs at least two forklift operators to move enough inventory around for the day, and have a total of five forklifts available, then your lower and upper bounds for the "Operate Forklift" task are 2 and 5, respectively.

For each worker, the required inputs are a "cost" of assigning the worker to each task. Costs can take any numeric value, and (since the approach taken is min cost/max flow) lower costs are considered better than higher costs. The application will interpret missing/blank entries for these costs as infeasible assignments; e.g. if a task requires some particular certification, you can leave that entry blank for all workers who are not certified.

Currently, the only supported file format is comma-separated values. See the io::csv mod for details on how the tables are set up, and src/io/csv/test-data for examples of input files.

## Min Cost/Max Flow Approach
While the "standard" [assignment problem](https://en.wikipedia.org/wiki/Assignment_problem) is typically handled via min weight matching, this application supports generalizations for which the same approach will not work - namely, a many-to-one relationship between workers and tasks and minimum requirements on tasks. Thus, a network flows approach is used instead.

### Network Representation
As in the standard problem, we represent each worker and each task as a node, with directed arcs connecting workers to tasks. We can push flow along these arcs to represent assigning workers to tasks. Thus, these arcs take the "costs" from the input. Since workers can be assigned to tasks at most once, the flow bounds on these arcs are [0, 1].

We also define two abstract nodes to represent an overall source and overall sink for the flow. The source connects to each worker node, and each task node connects to the sink. All arcs that touch these two nodes have zero cost, as the cost is fully represented within the arcs that pair workers to tasks. Arcs connecting the source to each worker have flow bounds of [0, 1] to represent the fact that workers can be assigned to at most one task each. Arcs connecting each task to the sink have flow bounds that match the task's min and max numbers of workers.

### Finding the Solution
To find min cost/max flow, we use minimum cost augmentation: find the shortest path from source to sink in the residual network, push flow along that path (updating the residual network accordingly), and repeat until no more paths are found.

To enforce minimum requirements on tasks, we split the min cost augmentation into two phases. In the first phase, we consider all arcs between the tasks and the sink to have flow bounds of [0, lower] instead of [lower, upper] so that the residual network is always in a feasible state. Once max flow is reached in phase 1, we flip each of those arcs (unless its lower and upper flow bounds are equal) and resume min cost augmentation with their actual flow bounds.

To enforce the constraint that all workers must be assigned to a task, we report an error if max flow is reached before all workers are assigned.
