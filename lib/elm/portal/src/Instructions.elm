module Instructions exposing (viewInstructions, viewFullPage)

import Element exposing (..)
import Element.Input
import Markdown

type alias Header =
    { header : String
    , content : List String
    }


type alias ParseResult =
    { value : Header
    , remaining : List String
    }

viewInstructions : (List String -> Maybe msg) -> msg -> msg -> String -> Element msg
viewInstructions onNext onViewFull onDone markdown =
    let
        root =
            parser (String.lines markdown)
    in
    case Markdown.viewMarkdown (String.join "\n" root.value.content) of
        Ok rendered ->
            Element.column [ spacing 20, height fill ] (List.append rendered [ 
                Element.row [ width fill  ] 
                    [ viewButton onNext onDone root.remaining 
                    , viewFullButton onViewFull
                    ]
            ])

        Err err ->
            Element.text err

viewFullPage :  String -> Element msg
viewFullPage markdown = 
 case Markdown.viewMarkdown markdown of
        Ok rendered ->
            Element.column [ spacing 20 ] rendered

        Err err ->
            Element.text err

viewButton : (List String -> Maybe msg) -> msg -> List String -> Element msg
viewButton onNext onDone remaining =
    if List.isEmpty remaining then
        Element.Input.button [] { onPress = Just onDone, label = Element.text "Done" }

    else
        Element.Input.button [] { onPress = onNext remaining, label = Element.text "Next" }

viewFullButton : msg -> Element msg 
viewFullButton onViewFull = 
    Element.Input.button [ alignRight ] { onPress = Just onViewFull, label = Element.text "View full page" }

headers : List String -> List ( Int, String )
headers file =
    let
        indexed =
            List.indexedMap Tuple.pair file
    in
    List.filter (\( _, a ) -> String.startsWith "#" a) indexed

parser : List String -> ParseResult
parser l =
    let
        ( b, _ ) =
            case List.head (List.drop 1 (headers l)) of
                Just v ->
                    v

                Nothing ->
                    ( List.length l, "" )
    in
    { value = parseHeader b l, remaining = parseRemaining b l }


parseHeader : Int -> List String -> Header
parseHeader b l =
    let
        content =
            List.indexedMap Tuple.pair l
                |> selectInside b
                |> List.map Tuple.second
    in
    { header =
        case List.head content of
            Just t ->
                t

            Nothing ->
                ""
    , content = content
    }


parseRemaining : Int -> List String -> List String
parseRemaining b l =
    List.indexedMap Tuple.pair l
        |> selectOutside b
        |> List.map Tuple.second


selectInside : Int -> List ( Int, String ) -> List ( Int, String )
selectInside b l =
    List.filter (\( i, _ ) -> i < b) l


selectOutside : Int -> List ( Int, String ) -> List ( Int, String )
selectOutside b l =
    List.filter (\( i, _ ) -> i >= b) l
