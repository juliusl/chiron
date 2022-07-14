module Instructions exposing (viewFullPage, viewInstructions, viewOutline)

import Element exposing (..)
import Element.Font as Font
import Element.Input
import Layout exposing (viewCommands)
import List exposing (isEmpty)
import Markdown


type alias Header =
    { header : String
    , content : List String
    }


type alias ParseResult =
    { value : Header
    , remaining : List String
    }


outline : String -> List ParseResult -> List ParseResult
outline markdown steps =
    let
        root =
            parser (String.lines markdown)
    in
    if isEmpty root.remaining then
        List.append steps [ root ]

    else
        outline (String.join "\n" root.remaining) (List.append steps [ root ])


viewOutline : (String -> msg) -> String -> Element msg
viewOutline onPress markdown =
    let
        steps =
            outline markdown []
    in
    viewCommands True
        (List.map
            (\item ->
                { onPress = onPress <| String.join "\n" (List.concat [ item.value.content, item.remaining ])
                , label = viewHeaderOutlineItem item.value.header
                }
            )
            steps
        )


viewHeaderOutlineItem : String -> Element msg
viewHeaderOutlineItem header =
    Element.el [ Font.size 14 ] <|
        Element.text <|
            (String.replace "#" "" <|
                String.replace "##" "\t" header
            )


viewInstructions : (String -> msg) -> (List String -> Maybe msg) -> msg -> msg -> String -> Element msg
viewInstructions onRunmd onNext onViewFull onDone markdown =
    let
        root =
            parser (String.lines markdown)
    in
    case Markdown.viewMarkdown onRunmd (String.join "\n" root.value.content) of
        Ok rendered ->
            Element.column [ spacing 20, height fill ]
                (List.append rendered
                    [ Element.row [ width (fill |> minimum 800) ]
                        [ viewButton onNext onDone root.remaining
                        , viewFullButton onViewFull
                        ]
                    ]
                )

        Err err ->
            Element.text err


viewFullPage : (String -> msg) -> String -> Element msg
viewFullPage onRunmd markdown =
    case Markdown.viewMarkdown onRunmd markdown of
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
