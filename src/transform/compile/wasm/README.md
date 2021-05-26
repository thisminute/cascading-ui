The runtime needs to know about the 3 kinds of groups in cwl - elements, classes, and listeners - and to be aware of 3 things we can do to each group - nothing, apply it to the DOM, or queue it to happen when an element matching some class is created later.

We do nothing to some groups because everything that needed to happen already happened during rendering and is in the static html file served with the page, but we can call that action "static" because we use the rendered static structure as a backbone of recursive calls to generating the runtime.

The function `static_elements` calls itself recursively and serves as the backbone of recursion through the tree, adding code to select an element to the code. The runtime uses a variable to keep track of the current element that we are operating on, and the only thing `static_elements` does is add code to update that variable to point at the element corresponding to the group we are currently processing. In other words, the first thing the runtime will do on load is begin to traverse the elements of the DOM from the root. As it stops at each element, it will process the elements, classes, and listeners of the group corresponding to that element, and recurse in a pattern that can switch between the apply and queue steps.

Consider these functions:

`document()` is the root of the recursion that generates the runtime

`static_elements()` generates code to select an element, and then apply all of its listeners to it and queue all of the classes that apply inside of it
`static_classes()` is just `queue_classes()`, so we don't have one
`static_listeners()` is just `apply_listeners()`, so we don't have one

`apply_elements()` generates code to create an element, select it, apply every element, class, and listener inside it, then deselect it, and attach the new element to the previously selected element
`apply_classes()` generates code to select every element that matches the class, and apply every element, class, and listener to them
`apply_listeners()` generates code that attaches a closure to the current element, which will trigger on an event. Triggering the closure will select the element that the closure was attached to, and then apply every element, class, and listener to it.

`queue_elements()` adds information about an element to a runtime object
`queue_classes()` is the root of the
`queue_listeners()` is

static_elements calls static_elements, queue_classes, and apply_listeners
queue_classes calls queue_elements, queue_classes, and queue_listeners
apply_listeners calls other apply functions
apply_classes calls queue_classes
