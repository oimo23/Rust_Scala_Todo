package com.example.todo

import akka.actor.typed.ActorSystem
import akka.actor.typed.scaladsl.Behaviors
import akka.http.scaladsl.Http
import akka.http.scaladsl.model.headers.{`Access-Control-Allow-Headers`, `Access-Control-Allow-Methods`, `Access-Control-Allow-Origin`}
import akka.http.scaladsl.model.{HttpMethods, StatusCodes}
import akka.http.scaladsl.server.Directives._
import akka.http.scaladsl.server.{Directive0, Route}

import scala.concurrent.ExecutionContext
import scala.util.{Failure, Success}

object Main extends App {
  implicit val system: ActorSystem[Nothing] = ActorSystem(Behaviors.empty, "todo-system")
  implicit val ec: ExecutionContext = system.executionContext

  val todoService = new TodoService()
  val todoRoutes = new TodoRoutes(todoService)

  val corsDirective: Directive0 = {
    respondWithHeaders(
      `Access-Control-Allow-Origin`.`*`,
      `Access-Control-Allow-Methods`(HttpMethods.GET, HttpMethods.POST, HttpMethods.PUT, HttpMethods.DELETE, HttpMethods.OPTIONS),
      `Access-Control-Allow-Headers`("Content-Type", "Authorization")
    )
  }

  val routes: Route = corsDirective {
    concat(
      options {
        complete(StatusCodes.OK)
      },
      todoRoutes.routes,
      pathSingleSlash {
        complete("Todo Scala API is running!")
      }
    )
  }

  val host = "0.0.0.0"
  val port = sys.env.getOrElse("PORT", "8080").toInt

  val bindingFuture = Http().newServerAt(host, port).bind(routes)

  bindingFuture.onComplete {
    case Success(binding) =>
      println(s"Server online at http://$host:$port/")
    case Failure(exception) =>
      println(s"Failed to bind HTTP server: ${exception.getMessage}")
      system.terminate()
  }
}