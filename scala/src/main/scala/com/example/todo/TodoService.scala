package com.example.todo

import scala.collection.mutable
import scala.concurrent.Future

class TodoService {
  private val todos = mutable.Map[Long, TodoItem]()
  private var nextId: Long = 1

  def getAllTodos(): Future[List[TodoItem]] = {
    Future.successful(todos.values.toList)
  }

  def getTodoById(id: Long): Future[Option[TodoItem]] = {
    Future.successful(todos.get(id))
  }

  def createTodo(todo: TodoItem): Future[TodoItem] = {
    val newTodo = todo.copy(id = Some(nextId))
    todos += (nextId -> newTodo)
    nextId += 1
    Future.successful(newTodo)
  }

  def updateTodo(id: Long, todo: TodoItem): Future[Option[TodoItem]] = {
    todos.get(id) match {
      case Some(_) =>
        val updatedTodo = todo.copy(id = Some(id))
        todos += (id -> updatedTodo)
        Future.successful(Some(updatedTodo))
      case None =>
        Future.successful(None)
    }
  }

  def deleteTodo(id: Long): Future[Boolean] = {
    val existed = todos.contains(id)
    todos.remove(id)
    Future.successful(existed)
  }
}