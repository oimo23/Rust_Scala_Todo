package com.example.todo

import akka.http.scaladsl.model.StatusCodes
import akka.http.scaladsl.server.Directives._
import akka.http.scaladsl.server.Route
import de.heikoseeberger.akkahttpcirce.FailFastCirceSupport._
import io.circe.generic.auto._

import scala.concurrent.ExecutionContext

class TodoRoutes(todoService: TodoService)(implicit ec: ExecutionContext) {

  val routes: Route = pathPrefix("api" / "todos") {
    concat(
      pathEnd {
        concat(
          get {
            complete(todoService.getAllTodos().map(StatusCodes.OK -> _))
          },
          post {
            entity(as[TodoItem]) { todo =>
              complete(todoService.createTodo(todo).map(StatusCodes.Created -> _))
            }
          }
        )
      },
      path(LongNumber) { id =>
        concat(
          get {
            onSuccess(todoService.getTodoById(id)) {
              case Some(todo) => complete(StatusCodes.OK -> todo)
              case None => complete(StatusCodes.NotFound)
            }
          },
          put {
            entity(as[TodoItem]) { todo =>
              onSuccess(todoService.updateTodo(id, todo)) {
                case Some(updatedTodo) => complete(StatusCodes.OK -> updatedTodo)
                case None => complete(StatusCodes.NotFound)
              }
            }
          },
          delete {
            onSuccess(todoService.deleteTodo(id)) { deleted =>
              if (deleted) complete(StatusCodes.NoContent)
              else complete(StatusCodes.NotFound)
            }
          }
        )
      }
    )
  }
}