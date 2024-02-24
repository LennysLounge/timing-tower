* A `leaf` or `dir` could always return a response. That would make the api for the caller easier to use instead of
having to check if the response is actually there.

* A mechanism to find out the current parent id would be nice.

* A drag drop action could also include the DropPosition of the dragged node.
This would be useful to undo / redo actions to be able to reverse the drop.

* Set where to show the drag indicator using the response.
* Dropping before and after the dragged node does not make sense but should still be
reported as an action to allow a user to remove the drop marker.