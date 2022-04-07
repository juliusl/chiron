module Instructions exposing (viewInstructions)

import Markdown
import Element exposing (..)
import Element.Input

viewInstructions: (List String -> Maybe msg) -> msg -> String -> Element msg
viewInstructions onNext onDone markdown =
  let
      root = parser (String.lines markdown)
  in
      case Markdown.viewMarkdown (String.join "\n" root.value.content) of 
        Ok rendered ->
          Element.column [spacing 20] (List.append rendered [ viewButton onNext onDone root.remaining ])
        Err err ->
          Element.text err

viewButton: (List String -> Maybe msg) -> msg -> List String -> Element msg 
viewButton onNext onDone remaining = 
  if List.isEmpty remaining then
    Element.Input.button [] { onPress = Just onDone, label = Element.text "Done" }
  else 
    Element.Input.button [] { onPress = onNext remaining, label = Element.text "Next" }

type alias Header 
  = { header: String
    , content: List String
    }

type alias ParseResult
  = { value: Header
    , remaining: List String
    }

headers: List String -> List (Int, String)
headers file =
  let
    indexed = List.indexedMap Tuple.pair file
  in
    List.filter (\(_, a) -> String.startsWith "#" a) indexed
    
next: ParseResult -> Maybe ParseResult
next r = 
  let 
    remaining = r.remaining
  in
    if (List.isEmpty remaining) then 
      Nothing 
    else
      Just (parser remaining)

parser: List String -> ParseResult
parser l =
  let 
    (b, _) = (case (List.head (List.drop 1 (headers l))) of 
                  Just v ->
                    v
                  Nothing ->
                    ((List.length l), ""))
  in
    { value = parseHeader b l, remaining = parseRemaining b l }
  

parseHeader: Int -> List String -> Header 
parseHeader b l =
  let
    content = List.indexedMap Tuple.pair l
                |> selectInside b
                |> List.map Tuple.second
  in
    { header = (case (List.head content) of 
                  Just t ->
                    t
                  Nothing ->
                    "")
    , content = content
    }

parseRemaining: Int -> List String -> List String
parseRemaining b l =
  List.indexedMap Tuple.pair l
    |> selectOutside b
    |> List.map Tuple.second
    
selectInside: Int -> List (Int, String) -> List (Int, String)
selectInside b l = 
  List.filter (\(i, _) -> i < b) l
  
selectOutside: Int -> List (Int, String) -> List (Int, String)
selectOutside b l = 
  List.filter (\(i, _) -> i >= b) l
