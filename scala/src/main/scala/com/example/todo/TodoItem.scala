package com.example.todo

import io.circe.{Decoder, Encoder}
import io.circe.generic.semiauto.{deriveDecoder, deriveEncoder}

case class TodoItem(
  id: Option[Long] = None,
  title: String,
  description: Option[String] = None,
  completed: Boolean = false
)

object TodoItem {
  implicit val todoItemEncoder: Encoder[TodoItem] = deriveEncoder[TodoItem]
  implicit val todoItemDecoder: Decoder[TodoItem] = deriveDecoder[TodoItem]
}