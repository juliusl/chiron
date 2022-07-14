port module Main exposing (..)

import Browser
import Editor exposing (viewCodeEditor)
import Element exposing (..)
import Element.Border as Border
import Element.Font as Font
import Html exposing (..)
import Http
import Instructions
import Json.Decode exposing (Decoder, field, list, map2, string)
import Layout exposing (view, viewCommands)
import List exposing (isEmpty)


type alias Model =
    { editor : Editor
    , labStatus : LabStatus
    , instructions : String
    , viewFull : Bool
    , edit : Bool
    , labs : List String
    , labName : String
    }


type alias Editor =
    { text : String
    , language : String
    , saved : String
    }


type alias LabStatus =
    { overview : String
    , expectations : List String
    }


type Msg
    = ResetText
    | Dispatch String
    | DispatchRunmd String
    | Save String
    | Instructions String
    | ViewFull
    | Edit
    | OpenLab String
    | CheckStatus
    | GotLab (Result Http.Error String)
    | GotLabs (Result Http.Error String)
    | GotLabStatus (Result Http.Error LabStatus)
    | Done


main : Program (Maybe String) Model Msg
main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }



-- Init


init : Maybe String -> ( Model, Cmd Msg )
init maybelab =
    let
        default =
            Model
                { text = ""
                , language = "markdown"
                , saved = ""
                }
                { overview = "This is placeholder text for a short-summary about what this lab will cover"
                , expectations = []
                }
                ""
                False
                False
                []
                ""
    in
    case maybelab of
        Just lab ->
            ( { default | labName = lab }, getLab GotLab lab )

        Nothing ->
            ( default, getLab GotLab "" )



-- VIEW


view : Model -> Browser.Document Msg
view model =
    let
        enableFullView =
            model.viewFull

        enableEdit =
            model.edit

        instructions =
            model.instructions

        editorMessages =
            { onDispatchSave = Dispatch "save", onSave = Save }

        editorSettings =
            { enableMonaco = True, visible = enableEdit }

        editorModel =
            { language = model.editor.language, text = model.editor.text }

        workspace =
            if enableEdit then
                viewCodeEditor editorMessages editorSettings editorModel

            else
                Element.column [ spacing 50 ]
                    [ Element.column [ spacing 12, width fill ]
                        [ Element.el [ Font.size 14 ] <|
                            Element.text <|
                                String.concat <|
                                    [ "Lab", " - ", model.labName ]
                        , Element.el
                            [ Font.size 14
                            ]
                          <|
                            Element.paragraph
                                [ Border.widthEach { top = 0, right = 1, bottom = 0, left = 0 }
                                , paddingEach { top = 4, right = 14, left = 14, bottom = 4 }
                                , Border.color (Element.rgb255 145 145 145)
                                ]
                                [ Element.text model.labStatus.overview ]
                        ]
                    , Element.column [ spacing 8, width fill ]
                        [ Element.el [ Font.size 14 ] <| Element.text "Requirements"
                        , Instructions.viewExpectations CheckStatus model.labStatus.expectations
                        ]
                    , Element.column [ spacing 8, width fill ]
                        [ Element.el [ Font.size 14 ] <| Element.text "Outline"
                        , Instructions.viewOutline Instructions model.editor.text
                        ]
                    ]

        labLinks =
            List.map
                (\lab ->
                    let
                        labName =
                            String.replace "/.runmd" "" lab
                    in
                    { onPress = OpenLab labName, label = Element.text labName }
                )
                model.labs
    in
    { title = "Chiron lab portal"
    , body =
        [ Layout.view
            { title = ""
            , showWorkspace = enableEdit
            , content =
                if enableFullView then
                    Instructions.viewFullPage onRunmd instructions

                else
                    Instructions.viewInstructions onRunmd onNext ViewFull Done instructions
            , workspace = workspace
            , actions =
                Element.column
                    [ spacing 50
                    ]
                    [ viewCommands False
                        [ { onPress = Edit, label = Element.text "Edit" }

                        -- TODO Add subcommands
                        -- , { onPress = (Dispatch "save"), label = ( Element.text "Render content" ) }
                        ]
                    , Element.column [ spacing 8 ]
                        [ Element.el [ Font.size 14 ] (Element.text "Labs")
                        , viewCommands False labLinks
                        ]
                    ]
            }
        ]
    }


onRunmd : String -> Msg
onRunmd runmd =
    DispatchRunmd runmd


onNext : List String -> Maybe Msg
onNext remaining =
    if List.isEmpty remaining then
        Nothing

    else
        Just (Instructions (String.join "\n" remaining))



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    let
        editor =
            model.editor
    in
    case msg of
        ResetText ->
            ( { model | editor = { editor | text = editor.saved } }, Cmd.none )

        Save content ->
            ( { model | editor = { editor | text = content, saved = content }, instructions = content }, Cmd.none )

        Dispatch cmd ->
            ( model, dispatchEditorCmd cmd )

        DispatchRunmd runmd ->
            ( model, dispatchRunmd runmd )

        Instructions instructions ->
            ( { model | instructions = instructions }, Cmd.none )

        ViewFull ->
            ( { model | editor = { editor | text = editor.saved }, instructions = model.editor.text, viewFull = True }, Cmd.none )

        Edit ->
            ( { model | edit = not model.edit }, Cmd.none )

        Done ->
            ( { model | instructions = model.editor.text }, Cmd.none )

        OpenLab name ->
            ( { model | labName = name, labs = [] }, getLab GotLab name )

        CheckStatus ->
            ( model, getLabStatus GotLabStatus model.labName )

        GotLabStatus result ->
            case result of
                Ok labStatus ->
                    ( { model | labStatus = labStatus }
                    , if isEmpty model.labs then
                        getLabs GotLabs

                      else
                        Cmd.none
                    )

                Err _ ->
                    ( model, Cmd.none )

        GotLab result ->
            case result of
                Ok lab ->
                    ( { model | editor = { editor | text = lab, saved = lab }, instructions = lab }, getLabStatus GotLabStatus model.labName )

                Err _ ->
                    ( model, Cmd.none )

        GotLabs result ->
            case result of
                Ok labs ->
                    ( { model | labs = List.filter (\name -> name /= String.concat [ model.labName, "/.runmd" ]) (String.split "\n" labs) }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )



-- SUBSCRIPTIONS
-- If Monaco is enabled, this will allow us to pipe commands to the editor


port dispatchEditorCmd : String -> Cmd msg



-- This is called by monaco to pass the current value of it's editor


port saveContent : (String -> msg) -> Sub msg



-- Dispatches runmd to the host


port dispatchRunmd : String -> Cmd msg


subscriptions : Model -> Sub Msg
subscriptions _ =
    saveContent Save



-- API


getLab : (Result Http.Error String -> msg) -> String -> Cmd msg
getLab msg lab =
    Http.get
        { url = String.concat [ "/lab/", lab ]
        , expect = Http.expectString msg
        }


getLabs : (Result Http.Error String -> msg) -> Cmd msg
getLabs msg =
    Http.get
        { url = "/labs"
        , expect = Http.expectString msg
        }


getLabStatus : (Result Http.Error LabStatus -> msg) -> String -> Cmd msg
getLabStatus msg lab =
    Http.get
        { url = String.concat [ "/lab/", lab, "/status" ]
        , expect = Http.expectJson msg labStatusDecoder
        }


labStatusDecoder : Decoder LabStatus
labStatusDecoder =
    map2 LabStatus
        (field "overview" string)
        (field "expectations" (list string))
