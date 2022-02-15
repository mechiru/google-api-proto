/// A set of words or phrases that represents a common concept likely to appear
/// in your audio, for example a list of passenger ship names. CustomClass items
/// can be substituted into placeholders that you set in PhraseSet phrases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomClass {
    /// The resource name of the custom class.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If this custom class is a resource, the custom_class_id is the resource id
    /// of the CustomClass. Case sensitive.
    #[prost(string, tag = "2")]
    pub custom_class_id: ::prost::alloc::string::String,
    /// A collection of class items.
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<custom_class::ClassItem>,
}
/// Nested message and enum types in `CustomClass`.
pub mod custom_class {
    /// An item of the class.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClassItem {
        /// The class item's value.
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
    }
}
/// Provides "hints" to the speech recognizer to favor specific words and phrases
/// in the results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhraseSet {
    /// The resource name of the phrase set.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of word and phrases.
    #[prost(message, repeated, tag = "2")]
    pub phrases: ::prost::alloc::vec::Vec<phrase_set::Phrase>,
    /// Hint Boost. Positive value will increase the probability that a specific
    /// phrase will be recognized over other similar sounding phrases. The higher
    /// the boost, the higher the chance of false positive recognition as well.
    /// Negative boost values would correspond to anti-biasing. Anti-biasing is not
    /// enabled, so negative boost will simply be ignored. Though `boost` can
    /// accept a wide range of positive values, most use cases are best served with
    /// values between 0 (exclusive) and 20. We recommend using a binary search
    /// approach to finding the optimal value for your use case. Speech recognition
    /// will skip PhraseSets with a boost value of 0.
    #[prost(float, tag = "4")]
    pub boost: f32,
}
/// Nested message and enum types in `PhraseSet`.
pub mod phrase_set {
    /// A phrases containing words and phrase "hints" so that
    /// the speech recognition is more likely to recognize them. This can be used
    /// to improve the accuracy for specific words and phrases, for example, if
    /// specific commands are typically spoken by the user. This can also be used
    /// to add additional words to the vocabulary of the recognizer. See
    /// [usage limits](<https://cloud.google.com/speech-to-text/quotas#content>).
    ///
    /// List items can also include pre-built or custom classes containing groups
    /// of words that represent common concepts that occur in natural language. For
    /// example, rather than providing a phrase hint for every month of the
    /// year (e.g. "i was born in january", "i was born in febuary", ...), use the
    /// pre-built `$MONTH` class improves the likelihood of correctly transcribing
    /// audio that includes months (e.g. "i was born in $month").
    /// To refer to pre-built classes, use the class' symbol prepended with `$`
    /// e.g. `$MONTH`. To refer to custom classes that were defined inline in the
    /// request, set the class's `custom_class_id` to a string unique to all class
    /// resources and inline classes. Then use the class' id wrapped in $`{...}`
    /// e.g. "${my-months}". To refer to custom classes resources, use the class'
    /// id wrapped in `${}` (e.g. `${my-months}`).
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Phrase {
        /// The phrase itself.
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
        /// Hint Boost. Overrides the boost set at the phrase set level.
        /// Positive value will increase the probability that a specific phrase will
        /// be recognized over other similar sounding phrases. The higher the boost,
        /// the higher the chance of false positive recognition as well. Negative
        /// boost will simply be ignored. Though `boost` can accept a wide range of
        /// positive values, most use cases are best served
        /// with values between 0 and 20. We recommend using a binary search approach
        /// to finding the optimal value for your use case. Speech recognition
        /// will skip PhraseSets with a boost value of 0.
        #[prost(float, tag = "2")]
        pub boost: f32,
    }
}
/// Speech adaptation configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechAdaptation {
    /// A collection of phrase sets. To specify the hints inline, leave the
    /// phrase set's `name` blank and fill in the rest of its fields. Any
    /// phrase set can use any custom class.
    #[prost(message, repeated, tag = "1")]
    pub phrase_sets: ::prost::alloc::vec::Vec<PhraseSet>,
    /// A collection of phrase set resource names to use.
    #[prost(string, repeated, tag = "2")]
    pub phrase_set_references: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A collection of custom classes. To specify the classes inline, leave the
    /// class' `name` blank and fill in the rest of its fields, giving it a unique
    /// `custom_class_id`. Refer to the inline defined class in phrase hints by its
    /// `custom_class_id`.
    #[prost(message, repeated, tag = "3")]
    pub custom_classes: ::prost::alloc::vec::Vec<CustomClass>,
}
/// Transcription normalization configuration. Use transcription normalization
/// to automatically replace parts of the transcript with phrases of your
/// choosing. For StreamingRecognize, this normalization only applies to stable
/// partial transcripts (stability > 0.8) and final transcripts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranscriptNormalization {
    /// A list of replacement entries. We will perform replacement with one entry
    /// at a time. For example, the second entry in ["cat" => "dog", "mountain cat"
    /// => "mountain dog"] will never be applied because we will always process the
    /// first entry before it. At most 100 entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<transcript_normalization::Entry>,
}
/// Nested message and enum types in `TranscriptNormalization`.
pub mod transcript_normalization {
    /// A single replacement configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        /// What to replace. Max length is 100 characters.
        #[prost(string, tag = "1")]
        pub search: ::prost::alloc::string::String,
        /// What to replace with. Max length is 100 characters.
        #[prost(string, tag = "2")]
        pub replace: ::prost::alloc::string::String,
        /// Whether the search is case sensitive.
        #[prost(bool, tag = "3")]
        pub case_sensitive: bool,
    }
}
/// The top-level message sent by the client for the `Recognize` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognizeRequest {
    /// Required. Provides information to the recognizer that specifies how to
    /// process the request.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<RecognitionConfig>,
    /// Required. The audio data to be recognized.
    #[prost(message, optional, tag = "2")]
    pub audio: ::core::option::Option<RecognitionAudio>,
}
/// The top-level message sent by the client for the `LongRunningRecognize`
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongRunningRecognizeRequest {
    /// Required. Provides information to the recognizer that specifies how to
    /// process the request.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<RecognitionConfig>,
    /// Required. The audio data to be recognized.
    #[prost(message, optional, tag = "2")]
    pub audio: ::core::option::Option<RecognitionAudio>,
    /// Optional. Specifies an optional destination for the recognition results.
    #[prost(message, optional, tag = "4")]
    pub output_config: ::core::option::Option<TranscriptOutputConfig>,
}
/// Specifies an optional destination for the recognition results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranscriptOutputConfig {
    #[prost(oneof = "transcript_output_config::OutputType", tags = "1")]
    pub output_type: ::core::option::Option<transcript_output_config::OutputType>,
}
/// Nested message and enum types in `TranscriptOutputConfig`.
pub mod transcript_output_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OutputType {
        /// Specifies a Cloud Storage URI for the recognition results. Must be
        /// specified in the format: `gs://bucket_name/object_name`, and the bucket
        /// must already exist.
        #[prost(string, tag = "1")]
        GcsUri(::prost::alloc::string::String),
    }
}
/// The top-level message sent by the client for the `StreamingRecognize` method.
/// Multiple `StreamingRecognizeRequest` messages are sent. The first message
/// must contain a `streaming_config` message and must not contain
/// `audio_content`. All subsequent messages must contain `audio_content` and
/// must not contain a `streaming_config` message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognizeRequest {
    /// The streaming request, which is either a streaming config or audio content.
    #[prost(oneof = "streaming_recognize_request::StreamingRequest", tags = "1, 2")]
    pub streaming_request: ::core::option::Option<streaming_recognize_request::StreamingRequest>,
}
/// Nested message and enum types in `StreamingRecognizeRequest`.
pub mod streaming_recognize_request {
    /// The streaming request, which is either a streaming config or audio content.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamingRequest {
        /// Provides information to the recognizer that specifies how to process the
        /// request. The first `StreamingRecognizeRequest` message must contain a
        /// `streaming_config`  message.
        #[prost(message, tag = "1")]
        StreamingConfig(super::StreamingRecognitionConfig),
        /// The audio data to be recognized. Sequential chunks of audio data are sent
        /// in sequential `StreamingRecognizeRequest` messages. The first
        /// `StreamingRecognizeRequest` message must not contain `audio_content` data
        /// and all subsequent `StreamingRecognizeRequest` messages must contain
        /// `audio_content` data. The audio bytes must be encoded as specified in
        /// `RecognitionConfig`. Note: as with all bytes fields, proto buffers use a
        /// pure binary representation (not base64). See
        /// [content limits](<https://cloud.google.com/speech-to-text/quotas#content>).
        #[prost(bytes, tag = "2")]
        AudioContent(::prost::bytes::Bytes),
    }
}
/// Provides information to the recognizer that specifies how to process the
/// request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionConfig {
    /// Required. Provides information to the recognizer that specifies how to
    /// process the request.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<RecognitionConfig>,
    /// If `false` or omitted, the recognizer will perform continuous
    /// recognition (continuing to wait for and process audio even if the user
    /// pauses speaking) until the client closes the input stream (gRPC API) or
    /// until the maximum time limit has been reached. May return multiple
    /// `StreamingRecognitionResult`s with the `is_final` flag set to `true`.
    ///
    /// If `true`, the recognizer will detect a single spoken utterance. When it
    /// detects that the user has paused or stopped speaking, it will return an
    /// `END_OF_SINGLE_UTTERANCE` event and cease recognition. It will return no
    /// more than one `StreamingRecognitionResult` with the `is_final` flag set to
    /// `true`.
    ///
    /// The `single_utterance` field can only be used with specified models,
    /// otherwise an error is thrown. The `model` field in \[`RecognitionConfig`][\]
    /// must be set to:
    ///
    /// * `command_and_search`
    /// * `phone_call` AND additional field `useEnhanced`=`true`
    /// * The `model` field is left undefined. In this case the API auto-selects
    ///   a model based on any other parameters that you set in
    ///   `RecognitionConfig`.
    #[prost(bool, tag = "2")]
    pub single_utterance: bool,
    /// If `true`, interim results (tentative hypotheses) may be
    /// returned as they become available (these interim results are indicated with
    /// the `is_final=false` flag).
    /// If `false` or omitted, only `is_final=true` result(s) are returned.
    #[prost(bool, tag = "3")]
    pub interim_results: bool,
}
/// Provides information to the recognizer that specifies how to process the
/// request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionConfig {
    /// Encoding of audio data sent in all `RecognitionAudio` messages.
    /// This field is optional for `FLAC` and `WAV` audio files and required
    /// for all other audio formats. For details, see \[AudioEncoding][google.cloud.speech.v1p1beta1.RecognitionConfig.AudioEncoding\].
    #[prost(enumeration = "recognition_config::AudioEncoding", tag = "1")]
    pub encoding: i32,
    /// Sample rate in Hertz of the audio data sent in all
    /// `RecognitionAudio` messages. Valid values are: 8000-48000.
    /// 16000 is optimal. For best results, set the sampling rate of the audio
    /// source to 16000 Hz. If that's not possible, use the native sample rate of
    /// the audio source (instead of re-sampling).
    /// This field is optional for FLAC and WAV audio files, but is
    /// required for all other audio formats. For details, see \[AudioEncoding][google.cloud.speech.v1p1beta1.RecognitionConfig.AudioEncoding\].
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// The number of channels in the input audio data.
    /// ONLY set this for MULTI-CHANNEL recognition.
    /// Valid values for LINEAR16 and FLAC are `1`-`8`.
    /// Valid values for OGG_OPUS are '1'-'254'.
    /// Valid value for MULAW, AMR, AMR_WB and SPEEX_WITH_HEADER_BYTE is only `1`.
    /// If `0` or omitted, defaults to one channel (mono).
    /// Note: We only recognize the first channel by default.
    /// To perform independent recognition on each channel set
    /// `enable_separate_recognition_per_channel` to 'true'.
    #[prost(int32, tag = "7")]
    pub audio_channel_count: i32,
    /// This needs to be set to `true` explicitly and `audio_channel_count` > 1
    /// to get each channel recognized separately. The recognition result will
    /// contain a `channel_tag` field to state which channel that result belongs
    /// to. If this is not true, we will only recognize the first channel. The
    /// request is billed cumulatively for all channels recognized:
    /// `audio_channel_count` multiplied by the length of the audio.
    #[prost(bool, tag = "12")]
    pub enable_separate_recognition_per_channel: bool,
    /// Required. The language of the supplied audio as a
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag.
    /// Example: "en-US".
    /// See [Language
    /// Support](<https://cloud.google.com/speech-to-text/docs/languages>) for a list
    /// of the currently supported language codes.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// A list of up to 3 additional
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tags,
    /// listing possible alternative languages of the supplied audio.
    /// See [Language
    /// Support](<https://cloud.google.com/speech-to-text/docs/languages>) for a list
    /// of the currently supported language codes. If alternative languages are
    /// listed, recognition result will contain recognition in the most likely
    /// language detected including the main language_code. The recognition result
    /// will include the language tag of the language detected in the audio. Note:
    /// This feature is only supported for Voice Command and Voice Search use cases
    /// and performance may vary for other use cases (e.g., phone call
    /// transcription).
    #[prost(string, repeated, tag = "18")]
    pub alternative_language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Maximum number of recognition hypotheses to be returned.
    /// Specifically, the maximum number of `SpeechRecognitionAlternative` messages
    /// within each `SpeechRecognitionResult`.
    /// The server may return fewer than `max_alternatives`.
    /// Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of
    /// one. If omitted, will return a maximum of one.
    #[prost(int32, tag = "4")]
    pub max_alternatives: i32,
    /// If set to `true`, the server will attempt to filter out
    /// profanities, replacing all but the initial character in each filtered word
    /// with asterisks, e.g. "f***". If set to `false` or omitted, profanities
    /// won't be filtered out.
    #[prost(bool, tag = "5")]
    pub profanity_filter: bool,
    /// Speech adaptation configuration improves the accuracy of speech
    /// recognition. For more information, see the [speech
    /// adaptation](<https://cloud.google.com/speech-to-text/docs/adaptation>)
    /// documentation.
    /// When speech adaptation is set it supersedes the `speech_contexts` field.
    #[prost(message, optional, tag = "20")]
    pub adaptation: ::core::option::Option<SpeechAdaptation>,
    /// Use transcription normalization to automatically replace parts of the
    /// transcript with phrases of your choosing. For StreamingRecognize, this
    /// normalization only applies to stable partial transcripts (stability > 0.8)
    /// and final transcripts.
    #[prost(message, optional, tag = "24")]
    pub transcript_normalization: ::core::option::Option<TranscriptNormalization>,
    /// Array of \[SpeechContext][google.cloud.speech.v1p1beta1.SpeechContext\].
    /// A means to provide context to assist the speech recognition. For more
    /// information, see
    /// [speech
    /// adaptation](<https://cloud.google.com/speech-to-text/docs/adaptation>).
    #[prost(message, repeated, tag = "6")]
    pub speech_contexts: ::prost::alloc::vec::Vec<SpeechContext>,
    /// If `true`, the top result includes a list of words and
    /// the start and end time offsets (timestamps) for those words. If
    /// `false`, no word-level time offset information is returned. The default is
    /// `false`.
    #[prost(bool, tag = "8")]
    pub enable_word_time_offsets: bool,
    /// If `true`, the top result includes a list of words and the
    /// confidence for those words. If `false`, no word-level confidence
    /// information is returned. The default is `false`.
    #[prost(bool, tag = "15")]
    pub enable_word_confidence: bool,
    /// If 'true', adds punctuation to recognition result hypotheses.
    /// This feature is only available in select languages. Setting this for
    /// requests in other languages has no effect at all.
    /// The default 'false' value does not add punctuation to result hypotheses.
    #[prost(bool, tag = "11")]
    pub enable_automatic_punctuation: bool,
    /// The spoken punctuation behavior for the call
    /// If not set, uses default behavior based on model of choice
    /// e.g. command_and_search will enable spoken punctuation by default
    /// If 'true', replaces spoken punctuation with the corresponding symbols in
    /// the request. For example, "how are you question mark" becomes "how are
    /// you?". See <https://cloud.google.com/speech-to-text/docs/spoken-punctuation>
    /// for support. If 'false', spoken punctuation is not replaced.
    #[prost(message, optional, tag = "22")]
    pub enable_spoken_punctuation: ::core::option::Option<bool>,
    /// The spoken emoji behavior for the call
    /// If not set, uses default behavior based on model of choice
    /// If 'true', adds spoken emoji formatting for the request. This will replace
    /// spoken emojis with the corresponding Unicode symbols in the final
    /// transcript. If 'false', spoken emojis are not replaced.
    #[prost(message, optional, tag = "23")]
    pub enable_spoken_emojis: ::core::option::Option<bool>,
    /// If 'true', enables speaker detection for each recognized word in
    /// the top alternative of the recognition result using a speaker_tag provided
    /// in the WordInfo.
    /// Note: Use diarization_config instead.
    #[deprecated]
    #[prost(bool, tag = "16")]
    pub enable_speaker_diarization: bool,
    /// If set, specifies the estimated number of speakers in the conversation.
    /// Defaults to '2'. Ignored unless enable_speaker_diarization is set to true.
    /// Note: Use diarization_config instead.
    #[deprecated]
    #[prost(int32, tag = "17")]
    pub diarization_speaker_count: i32,
    /// Config to enable speaker diarization and set additional
    /// parameters to make diarization better suited for your application.
    /// Note: When this is enabled, we send all the words from the beginning of the
    /// audio for the top alternative in every consecutive STREAMING responses.
    /// This is done in order to improve our speaker tags as our models learn to
    /// identify the speakers in the conversation over time.
    /// For non-streaming requests, the diarization results will be provided only
    /// in the top alternative of the FINAL SpeechRecognitionResult.
    #[prost(message, optional, tag = "19")]
    pub diarization_config: ::core::option::Option<SpeakerDiarizationConfig>,
    /// Metadata regarding this request.
    #[prost(message, optional, tag = "9")]
    pub metadata: ::core::option::Option<RecognitionMetadata>,
    /// Which model to select for the given request. Select the model
    /// best suited to your domain to get best results. If a model is not
    /// explicitly specified, then we auto-select a model based on the parameters
    /// in the RecognitionConfig.
    /// <table>
    ///   <tr>
    ///     <td><b>Model</b></td>
    ///     <td><b>Description</b></td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>command_and_search</code></td>
    ///     <td>Best for short queries such as voice commands or voice search.</td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>phone_call</code></td>
    ///     <td>Best for audio that originated from a phone call (typically
    ///     recorded at an 8khz sampling rate).</td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>video</code></td>
    ///     <td>Best for audio that originated from video or includes multiple
    ///         speakers. Ideally the audio is recorded at a 16khz or greater
    ///         sampling rate. This is a premium model that costs more than the
    ///         standard rate.</td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>default</code></td>
    ///     <td>Best for audio that is not one of the specific audio models.
    ///         For example, long-form audio. Ideally the audio is high-fidelity,
    ///         recorded at a 16khz or greater sampling rate.</td>
    ///   </tr>
    /// </table>
    #[prost(string, tag = "13")]
    pub model: ::prost::alloc::string::String,
    /// Set to true to use an enhanced model for speech recognition.
    /// If `use_enhanced` is set to true and the `model` field is not set, then
    /// an appropriate enhanced model is chosen if an enhanced model exists for
    /// the audio.
    ///
    /// If `use_enhanced` is true and an enhanced version of the specified model
    /// does not exist, then the speech is recognized using the standard version
    /// of the specified model.
    #[prost(bool, tag = "14")]
    pub use_enhanced: bool,
}
/// Nested message and enum types in `RecognitionConfig`.
pub mod recognition_config {
    /// The encoding of the audio data sent in the request.
    ///
    /// All encodings support only 1 channel (mono) audio, unless the
    /// `audio_channel_count` and `enable_separate_recognition_per_channel` fields
    /// are set.
    ///
    /// For best results, the audio source should be captured and transmitted using
    /// a lossless encoding (`FLAC` or `LINEAR16`). The accuracy of the speech
    /// recognition can be reduced if lossy codecs are used to capture or transmit
    /// audio, particularly if background noise is present. Lossy codecs include
    /// `MULAW`, `AMR`, `AMR_WB`, `OGG_OPUS`, `SPEEX_WITH_HEADER_BYTE`, `MP3`,
    /// and `WEBM_OPUS`.
    ///
    /// The `FLAC` and `WAV` audio file formats include a header that describes the
    /// included audio content. You can request recognition for `WAV` files that
    /// contain either `LINEAR16` or `MULAW` encoded audio.
    /// If you send `FLAC` or `WAV` audio file format in
    /// your request, you do not need to specify an `AudioEncoding`; the audio
    /// encoding format is determined from the file header. If you specify
    /// an `AudioEncoding` when you send  send `FLAC` or `WAV` audio, the
    /// encoding configuration must match the encoding described in the audio
    /// header; otherwise the request returns an
    /// \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] error code.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AudioEncoding {
        /// Not specified.
        EncodingUnspecified = 0,
        /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
        Linear16 = 1,
        /// `FLAC` (Free Lossless Audio
        /// Codec) is the recommended encoding because it is
        /// lossless--therefore recognition is not compromised--and
        /// requires only about half the bandwidth of `LINEAR16`. `FLAC` stream
        /// encoding supports 16-bit and 24-bit samples, however, not all fields in
        /// `STREAMINFO` are supported.
        Flac = 2,
        /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
        Mulaw = 3,
        /// Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
        Amr = 4,
        /// Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
        AmrWb = 5,
        /// Opus encoded audio frames in Ogg container
        /// (\[OggOpus\](<https://wiki.xiph.org/OggOpus>)).
        /// `sample_rate_hertz` must be one of 8000, 12000, 16000, 24000, or 48000.
        OggOpus = 6,
        /// Although the use of lossy encodings is not recommended, if a very low
        /// bitrate encoding is required, `OGG_OPUS` is highly preferred over
        /// Speex encoding. The \[Speex\](<https://speex.org/>)  encoding supported by
        /// Cloud Speech API has a header byte in each block, as in MIME type
        /// `audio/x-speex-with-header-byte`.
        /// It is a variant of the RTP Speex encoding defined in
        /// [RFC 5574](<https://tools.ietf.org/html/rfc5574>).
        /// The stream is a sequence of blocks, one block per RTP packet. Each block
        /// starts with a byte containing the length of the block, in bytes, followed
        /// by one or more frames of Speex data, padded to an integral number of
        /// bytes (octets) as specified in RFC 5574. In other words, each RTP header
        /// is replaced with a single byte containing the block length. Only Speex
        /// wideband is supported. `sample_rate_hertz` must be 16000.
        SpeexWithHeaderByte = 7,
        /// MP3 audio. MP3 encoding is a Beta feature and only available in
        /// v1p1beta1. Support all standard MP3 bitrates (which range from 32-320
        /// kbps). When using this encoding, `sample_rate_hertz` has to match the
        /// sample rate of the file being used.
        Mp3 = 8,
        /// Opus encoded audio frames in WebM container
        /// (\[OggOpus\](<https://wiki.xiph.org/OggOpus>)). `sample_rate_hertz` must be
        /// one of 8000, 12000, 16000, 24000, or 48000.
        WebmOpus = 9,
    }
}
/// Config to enable speaker diarization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeakerDiarizationConfig {
    /// If 'true', enables speaker detection for each recognized word in
    /// the top alternative of the recognition result using a speaker_tag provided
    /// in the WordInfo.
    #[prost(bool, tag = "1")]
    pub enable_speaker_diarization: bool,
    /// Minimum number of speakers in the conversation. This range gives you more
    /// flexibility by allowing the system to automatically determine the correct
    /// number of speakers. If not set, the default value is 2.
    #[prost(int32, tag = "2")]
    pub min_speaker_count: i32,
    /// Maximum number of speakers in the conversation. This range gives you more
    /// flexibility by allowing the system to automatically determine the correct
    /// number of speakers. If not set, the default value is 6.
    #[prost(int32, tag = "3")]
    pub max_speaker_count: i32,
    /// Output only. Unused.
    #[deprecated]
    #[prost(int32, tag = "5")]
    pub speaker_tag: i32,
}
/// Description of audio data to be recognized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionMetadata {
    /// The use case most closely describing the audio content to be recognized.
    #[prost(enumeration = "recognition_metadata::InteractionType", tag = "1")]
    pub interaction_type: i32,
    /// The industry vertical to which this speech recognition request most
    /// closely applies. This is most indicative of the topics contained
    /// in the audio.  Use the 6-digit NAICS code to identify the industry
    /// vertical - see <https://www.naics.com/search/.>
    #[prost(uint32, tag = "3")]
    pub industry_naics_code_of_audio: u32,
    /// The audio type that most closely describes the audio being recognized.
    #[prost(enumeration = "recognition_metadata::MicrophoneDistance", tag = "4")]
    pub microphone_distance: i32,
    /// The original media the speech was recorded on.
    #[prost(enumeration = "recognition_metadata::OriginalMediaType", tag = "5")]
    pub original_media_type: i32,
    /// The type of device the speech was recorded with.
    #[prost(enumeration = "recognition_metadata::RecordingDeviceType", tag = "6")]
    pub recording_device_type: i32,
    /// The device used to make the recording.  Examples 'Nexus 5X' or
    /// 'Polycom SoundStation IP 6000' or 'POTS' or 'VoIP' or
    /// 'Cardioid Microphone'.
    #[prost(string, tag = "7")]
    pub recording_device_name: ::prost::alloc::string::String,
    /// Mime type of the original audio file.  For example `audio/m4a`,
    /// `audio/x-alaw-basic`, `audio/mp3`, `audio/3gpp`.
    /// A list of possible audio mime types is maintained at
    /// <http://www.iana.org/assignments/media-types/media-types.xhtml#audio>
    #[prost(string, tag = "8")]
    pub original_mime_type: ::prost::alloc::string::String,
    /// Obfuscated (privacy-protected) ID of the user, to identify number of
    /// unique users using the service.
    #[deprecated]
    #[prost(int64, tag = "9")]
    pub obfuscated_id: i64,
    /// Description of the content. Eg. "Recordings of federal supreme court
    /// hearings from 2012".
    #[prost(string, tag = "10")]
    pub audio_topic: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RecognitionMetadata`.
pub mod recognition_metadata {
    /// Use case categories that the audio recognition request can be described
    /// by.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InteractionType {
        /// Use case is either unknown or is something other than one of the other
        /// values below.
        Unspecified = 0,
        /// Multiple people in a conversation or discussion. For example in a
        /// meeting with two or more people actively participating. Typically
        /// all the primary people speaking would be in the same room (if not,
        /// see PHONE_CALL)
        Discussion = 1,
        /// One or more persons lecturing or presenting to others, mostly
        /// uninterrupted.
        Presentation = 2,
        /// A phone-call or video-conference in which two or more people, who are
        /// not in the same room, are actively participating.
        PhoneCall = 3,
        /// A recorded message intended for another person to listen to.
        Voicemail = 4,
        /// Professionally produced audio (eg. TV Show, Podcast).
        ProfessionallyProduced = 5,
        /// Transcribe spoken questions and queries into text.
        VoiceSearch = 6,
        /// Transcribe voice commands, such as for controlling a device.
        VoiceCommand = 7,
        /// Transcribe speech to text to create a written document, such as a
        /// text-message, email or report.
        Dictation = 8,
    }
    /// Enumerates the types of capture settings describing an audio file.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MicrophoneDistance {
        /// Audio type is not known.
        Unspecified = 0,
        /// The audio was captured from a closely placed microphone. Eg. phone,
        /// dictaphone, or handheld microphone. Generally if there speaker is within
        /// 1 meter of the microphone.
        Nearfield = 1,
        /// The speaker if within 3 meters of the microphone.
        Midfield = 2,
        /// The speaker is more than 3 meters away from the microphone.
        Farfield = 3,
    }
    /// The original media the speech was recorded on.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OriginalMediaType {
        /// Unknown original media type.
        Unspecified = 0,
        /// The speech data is an audio recording.
        Audio = 1,
        /// The speech data originally recorded on a video.
        Video = 2,
    }
    /// The type of device the speech was recorded with.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RecordingDeviceType {
        /// The recording device is unknown.
        Unspecified = 0,
        /// Speech was recorded on a smartphone.
        Smartphone = 1,
        /// Speech was recorded using a personal computer or tablet.
        Pc = 2,
        /// Speech was recorded over a phone line.
        PhoneLine = 3,
        /// Speech was recorded in a vehicle.
        Vehicle = 4,
        /// Speech was recorded outdoors.
        OtherOutdoorDevice = 5,
        /// Speech was recorded indoors.
        OtherIndoorDevice = 6,
    }
}
/// Provides "hints" to the speech recognizer to favor specific words and phrases
/// in the results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechContext {
    /// A list of strings containing words and phrases "hints" so that
    /// the speech recognition is more likely to recognize them. This can be used
    /// to improve the accuracy for specific words and phrases, for example, if
    /// specific commands are typically spoken by the user. This can also be used
    /// to add additional words to the vocabulary of the recognizer. See
    /// [usage limits](<https://cloud.google.com/speech-to-text/quotas#content>).
    ///
    /// List items can also be set to classes for groups of words that represent
    /// common concepts that occur in natural language. For example, rather than
    /// providing phrase hints for every month of the year, using the $MONTH class
    /// improves the likelihood of correctly transcribing audio that includes
    /// months.
    #[prost(string, repeated, tag = "1")]
    pub phrases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Hint Boost. Positive value will increase the probability that a specific
    /// phrase will be recognized over other similar sounding phrases. The higher
    /// the boost, the higher the chance of false positive recognition as well.
    /// Negative boost values would correspond to anti-biasing. Anti-biasing is not
    /// enabled, so negative boost will simply be ignored. Though `boost` can
    /// accept a wide range of positive values, most use cases are best served with
    /// values between 0 and 20. We recommend using a binary search approach to
    /// finding the optimal value for your use case.
    #[prost(float, tag = "4")]
    pub boost: f32,
}
/// Contains audio data in the encoding specified in the `RecognitionConfig`.
/// Either `content` or `uri` must be supplied. Supplying both or neither
/// returns \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]. See
/// [content limits](<https://cloud.google.com/speech-to-text/quotas#content>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionAudio {
    /// The audio source, which is either inline content or a Google Cloud
    /// Storage uri.
    #[prost(oneof = "recognition_audio::AudioSource", tags = "1, 2")]
    pub audio_source: ::core::option::Option<recognition_audio::AudioSource>,
}
/// Nested message and enum types in `RecognitionAudio`.
pub mod recognition_audio {
    /// The audio source, which is either inline content or a Google Cloud
    /// Storage uri.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AudioSource {
        /// The audio data bytes encoded as specified in
        /// `RecognitionConfig`. Note: as with all bytes fields, proto buffers use a
        /// pure binary representation, whereas JSON representations use base64.
        #[prost(bytes, tag = "1")]
        Content(::prost::bytes::Bytes),
        /// URI that points to a file that contains audio data bytes as specified in
        /// `RecognitionConfig`. The file must not be compressed (for example, gzip).
        /// Currently, only Google Cloud Storage URIs are
        /// supported, which must be specified in the following format:
        /// `gs://bucket_name/object_name` (other URI formats return
        /// \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]). For more information, see
        /// [Request URIs](<https://cloud.google.com/storage/docs/reference-uris>).
        #[prost(string, tag = "2")]
        Uri(::prost::alloc::string::String),
    }
}
/// The only message returned to the client by the `Recognize` method. It
/// contains the result as zero or more sequential `SpeechRecognitionResult`
/// messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognizeResponse {
    /// Sequential list of transcription results corresponding to
    /// sequential portions of audio.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<SpeechRecognitionResult>,
    /// When available, billed audio seconds for the corresponding request.
    #[prost(message, optional, tag = "3")]
    pub total_billed_time: ::core::option::Option<::prost_types::Duration>,
}
/// The only message returned to the client by the `LongRunningRecognize` method.
/// It contains the result as zero or more sequential `SpeechRecognitionResult`
/// messages. It is included in the `result.response` field of the `Operation`
/// returned by the `GetOperation` call of the `google::longrunning::Operations`
/// service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongRunningRecognizeResponse {
    /// Sequential list of transcription results corresponding to
    /// sequential portions of audio.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<SpeechRecognitionResult>,
    /// When available, billed audio seconds for the corresponding request.
    #[prost(message, optional, tag = "3")]
    pub total_billed_time: ::core::option::Option<::prost_types::Duration>,
    /// Original output config if present in the request.
    #[prost(message, optional, tag = "6")]
    pub output_config: ::core::option::Option<TranscriptOutputConfig>,
    /// If the transcript output fails this field contains the relevant error.
    #[prost(message, optional, tag = "7")]
    pub output_error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Describes the progress of a long-running `LongRunningRecognize` call. It is
/// included in the `metadata` field of the `Operation` returned by the
/// `GetOperation` call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongRunningRecognizeMetadata {
    /// Approximate percentage of audio processed thus far. Guaranteed to be 100
    /// when the audio is fully processed and the results are available.
    #[prost(int32, tag = "1")]
    pub progress_percent: i32,
    /// Time when the request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time of the most recent processing update.
    #[prost(message, optional, tag = "3")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The URI of the audio file being transcribed. Empty if the audio was sent
    /// as byte content.
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    /// Output only. A copy of the TranscriptOutputConfig if it was set in the request.
    #[prost(message, optional, tag = "5")]
    pub output_config: ::core::option::Option<TranscriptOutputConfig>,
}
/// `StreamingRecognizeResponse` is the only message returned to the client by
/// `StreamingRecognize`. A series of zero or more `StreamingRecognizeResponse`
/// messages are streamed back to the client. If there is no recognizable
/// audio, and `single_utterance` is set to false, then no messages are streamed
/// back to the client.
///
/// Here's an example of a series of `StreamingRecognizeResponse`s that might be
/// returned while processing audio:
///
/// 1. results { alternatives { transcript: "tube" } stability: 0.01 }
///
/// 2. results { alternatives { transcript: "to be a" } stability: 0.01 }
///
/// 3. results { alternatives { transcript: "to be" } stability: 0.9 }
///    results { alternatives { transcript: " or not to be" } stability: 0.01 }
///
/// 4. results { alternatives { transcript: "to be or not to be"
///                             confidence: 0.92 }
///              alternatives { transcript: "to bee or not to bee" }
///              is_final: true }
///
/// 5. results { alternatives { transcript: " that's" } stability: 0.01 }
///
/// 6. results { alternatives { transcript: " that is" } stability: 0.9 }
///    results { alternatives { transcript: " the question" } stability: 0.01 }
///
/// 7. results { alternatives { transcript: " that is the question"
///                             confidence: 0.98 }
///              alternatives { transcript: " that was the question" }
///              is_final: true }
///
/// Notes:
///
/// - Only two of the above responses #4 and #7 contain final results; they are
///   indicated by `is_final: true`. Concatenating these together generates the
///   full transcript: "to be or not to be that is the question".
///
/// - The others contain interim `results`. #3 and #6 contain two interim
///   `results`: the first portion has a high stability and is less likely to
///   change; the second portion has a low stability and is very likely to
///   change. A UI designer might choose to show only high stability `results`.
///
/// - The specific `stability` and `confidence` values shown above are only for
///   illustrative purposes. Actual values may vary.
///
/// - In each response, only one of these fields will be set:
///     `error`,
///     `speech_event_type`, or
///     one or more (repeated) `results`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognizeResponse {
    /// If set, returns a \[google.rpc.Status][google.rpc.Status\] message that
    /// specifies the error for the operation.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// This repeated list contains zero or more results that
    /// correspond to consecutive portions of the audio currently being processed.
    /// It contains zero or one `is_final=true` result (the newly settled portion),
    /// followed by zero or more `is_final=false` results (the interim results).
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<StreamingRecognitionResult>,
    /// Indicates the type of speech event.
    #[prost(
        enumeration = "streaming_recognize_response::SpeechEventType",
        tag = "4"
    )]
    pub speech_event_type: i32,
    /// When available, billed audio seconds for the stream.
    /// Set only if this is the last response in the stream.
    #[prost(message, optional, tag = "5")]
    pub total_billed_time: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `StreamingRecognizeResponse`.
pub mod streaming_recognize_response {
    /// Indicates the type of speech event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpeechEventType {
        /// No speech event specified.
        SpeechEventUnspecified = 0,
        /// This event indicates that the server has detected the end of the user's
        /// speech utterance and expects no additional speech. Therefore, the server
        /// will not process additional audio (although it may subsequently return
        /// additional results). The client should stop sending additional audio
        /// data, half-close the gRPC connection, and wait for any additional results
        /// until the server closes the gRPC connection. This event is only sent if
        /// `single_utterance` was set to `true`, and is not used otherwise.
        EndOfSingleUtterance = 1,
    }
}
/// A streaming speech recognition result corresponding to a portion of the audio
/// that is currently being processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionResult {
    /// May contain one or more recognition hypotheses (up to the
    /// maximum specified in `max_alternatives`).
    /// These alternatives are ordered in terms of accuracy, with the top (first)
    /// alternative being the most probable, as ranked by the recognizer.
    #[prost(message, repeated, tag = "1")]
    pub alternatives: ::prost::alloc::vec::Vec<SpeechRecognitionAlternative>,
    /// If `false`, this `StreamingRecognitionResult` represents an
    /// interim result that may change. If `true`, this is the final time the
    /// speech service will return this particular `StreamingRecognitionResult`,
    /// the recognizer will not return any further hypotheses for this portion of
    /// the transcript and corresponding audio.
    #[prost(bool, tag = "2")]
    pub is_final: bool,
    /// An estimate of the likelihood that the recognizer will not
    /// change its guess about this interim result. Values range from 0.0
    /// (completely unstable) to 1.0 (completely stable).
    /// This field is only provided for interim results (`is_final=false`).
    /// The default of 0.0 is a sentinel value indicating `stability` was not set.
    #[prost(float, tag = "3")]
    pub stability: f32,
    /// Time offset of the end of this result relative to the
    /// beginning of the audio.
    #[prost(message, optional, tag = "4")]
    pub result_end_time: ::core::option::Option<::prost_types::Duration>,
    /// For multi-channel audio, this is the channel number corresponding to the
    /// recognized result for the audio from that channel.
    /// For audio_channel_count = N, its output values can range from '1' to 'N'.
    #[prost(int32, tag = "5")]
    pub channel_tag: i32,
    /// Output only. The \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag
    /// of the language in this result. This language code was detected to have
    /// the most likelihood of being spoken in the audio.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
}
/// A speech recognition result corresponding to a portion of the audio.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechRecognitionResult {
    /// May contain one or more recognition hypotheses (up to the
    /// maximum specified in `max_alternatives`).
    /// These alternatives are ordered in terms of accuracy, with the top (first)
    /// alternative being the most probable, as ranked by the recognizer.
    #[prost(message, repeated, tag = "1")]
    pub alternatives: ::prost::alloc::vec::Vec<SpeechRecognitionAlternative>,
    /// For multi-channel audio, this is the channel number corresponding to the
    /// recognized result for the audio from that channel.
    /// For audio_channel_count = N, its output values can range from '1' to 'N'.
    #[prost(int32, tag = "2")]
    pub channel_tag: i32,
    /// Time offset of the end of this result relative to the
    /// beginning of the audio.
    #[prost(message, optional, tag = "4")]
    pub result_end_time: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag
    /// of the language in this result. This language code was detected to have
    /// the most likelihood of being spoken in the audio.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Alternative hypotheses (a.k.a. n-best list).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechRecognitionAlternative {
    /// Transcript text representing the words that the user spoke.
    #[prost(string, tag = "1")]
    pub transcript: ::prost::alloc::string::String,
    /// The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative of a non-streaming
    /// result or, of a streaming result where `is_final=true`.
    /// This field is not guaranteed to be accurate and users should not rely on it
    /// to be always provided.
    /// The default of 0.0 is a sentinel value indicating `confidence` was not set.
    #[prost(float, tag = "2")]
    pub confidence: f32,
    /// A list of word-specific information for each recognized word.
    /// Note: When `enable_speaker_diarization` is true, you will see all the words
    /// from the beginning of the audio.
    #[prost(message, repeated, tag = "3")]
    pub words: ::prost::alloc::vec::Vec<WordInfo>,
}
/// Word-specific information for recognized words.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordInfo {
    /// Time offset relative to the beginning of the audio,
    /// and corresponding to the start of the spoken word.
    /// This field is only set if `enable_word_time_offsets=true` and only
    /// in the top hypothesis.
    /// This is an experimental feature and the accuracy of the time offset can
    /// vary.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Duration>,
    /// Time offset relative to the beginning of the audio,
    /// and corresponding to the end of the spoken word.
    /// This field is only set if `enable_word_time_offsets=true` and only
    /// in the top hypothesis.
    /// This is an experimental feature and the accuracy of the time offset can
    /// vary.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Duration>,
    /// The word corresponding to this set of information.
    #[prost(string, tag = "3")]
    pub word: ::prost::alloc::string::String,
    /// The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative of a non-streaming
    /// result or, of a streaming result where `is_final=true`.
    /// This field is not guaranteed to be accurate and users should not rely on it
    /// to be always provided.
    /// The default of 0.0 is a sentinel value indicating `confidence` was not set.
    #[prost(float, tag = "4")]
    pub confidence: f32,
    /// Output only. A distinct integer value is assigned for every speaker within
    /// the audio. This field specifies which one of those speakers was detected to
    /// have spoken this word. Value ranges from '1' to diarization_speaker_count.
    /// speaker_tag is set if enable_speaker_diarization = 'true' and only in the
    /// top alternative.
    #[prost(int32, tag = "5")]
    pub speaker_tag: i32,
}
#[doc = r" Generated client implementations."]
pub mod speech_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service that implements Google Cloud Speech API."]
    #[derive(Debug, Clone)]
    pub struct SpeechClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SpeechClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SpeechClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SpeechClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Performs synchronous speech recognition: receive results after all audio"]
        #[doc = " has been sent and processed."]
        pub async fn recognize(
            &mut self,
            request: impl tonic::IntoRequest<super::RecognizeRequest>,
        ) -> Result<tonic::Response<super::RecognizeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Speech/Recognize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs asynchronous speech recognition: receive results via the"]
        #[doc = " google.longrunning.Operations interface. Returns either an"]
        #[doc = " `Operation.error` or an `Operation.response` which contains"]
        #[doc = " a `LongRunningRecognizeResponse` message."]
        #[doc = " For more information on asynchronous speech recognition, see the"]
        #[doc = " [how-to](https://cloud.google.com/speech-to-text/docs/async-recognize)."]
        pub async fn long_running_recognize(
            &mut self,
            request: impl tonic::IntoRequest<super::LongRunningRecognizeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Speech/LongRunningRecognize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs bidirectional streaming speech recognition: receive results while"]
        #[doc = " sending audio. This method is only available via the gRPC API (not REST)."]
        pub async fn streaming_recognize(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingRecognizeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingRecognizeResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Speech/StreamingRecognize",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
/// Message sent by the client for the `CreatePhraseSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePhraseSetRequest {
    /// Required. The parent resource where this phrase set will be created. Format:
    ///
    /// `projects/{project}/locations/{location}/phraseSets`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the phrase set, which will become the final
    /// component of the phrase set's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    #[prost(string, tag = "2")]
    pub phrase_set_id: ::prost::alloc::string::String,
    /// Required. The phrase set to create.
    #[prost(message, optional, tag = "3")]
    pub phrase_set: ::core::option::Option<PhraseSet>,
}
/// Message sent by the client for the `UpdatePhraseSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhraseSetRequest {
    /// Required. The phrase set to update.
    ///
    /// The phrase set's `name` field is used to identify the set to be
    /// updated. Format:
    ///
    /// `projects/{project}/locations/{location}/phraseSets/{phrase_set}`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(message, optional, tag = "1")]
    pub phrase_set: ::core::option::Option<PhraseSet>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Message sent by the client for the `GetPhraseSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhraseSetRequest {
    /// Required. The name of the phrase set to retrieve. Format:
    ///
    /// `projects/{project}/locations/{location}/phraseSets/{phrase_set}`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message sent by the client for the `ListPhraseSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhraseSetRequest {
    /// Required. The parent, which owns this collection of phrase set. Format:
    ///
    /// `projects/{project}/locations/{location}`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of phrase sets to return. The service may return
    /// fewer than this value. If unspecified, at most 50 phrase sets will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListPhraseSet` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListPhraseSet` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message returned to the client by the `ListPhraseSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhraseSetResponse {
    /// The phrase set.
    #[prost(message, repeated, tag = "1")]
    pub phrase_sets: ::prost::alloc::vec::Vec<PhraseSet>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message sent by the client for the `DeletePhraseSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePhraseSetRequest {
    /// Required. The name of the phrase set to delete. Format:
    ///
    /// `projects/{project}/locations/{location}/phraseSets/{phrase_set}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message sent by the client for the `CreateCustomClass` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomClassRequest {
    /// Required. The parent resource where this custom class will be created. Format:
    ///
    /// `projects/{project}/locations/{location}/customClasses`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the custom class, which will become the final
    /// component of the custom class' resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    #[prost(string, tag = "2")]
    pub custom_class_id: ::prost::alloc::string::String,
    /// Required. The custom class to create.
    #[prost(message, optional, tag = "3")]
    pub custom_class: ::core::option::Option<CustomClass>,
}
/// Message sent by the client for the `UpdateCustomClass` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomClassRequest {
    /// Required. The custom class to update.
    ///
    /// The custom class's `name` field is used to identify the custom class to be
    /// updated. Format:
    ///
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(message, optional, tag = "1")]
    pub custom_class: ::core::option::Option<CustomClass>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Message sent by the client for the `GetCustomClass` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomClassRequest {
    /// Required. The name of the custom class to retrieve. Format:
    ///
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message sent by the client for the `ListCustomClasses` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomClassesRequest {
    /// Required. The parent, which owns this collection of custom classes. Format:
    ///
    /// `projects/{project}/locations/{location}/customClasses`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of custom classes to return. The service may return
    /// fewer than this value. If unspecified, at most 50 custom classes will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomClass` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomClass` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message returned to the client by the `ListCustomClasses` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomClassesResponse {
    /// The custom classes.
    #[prost(message, repeated, tag = "1")]
    pub custom_classes: ::prost::alloc::vec::Vec<CustomClass>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message sent by the client for the `DeleteCustomClass` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCustomClassRequest {
    /// Required. The name of the custom class to delete. Format:
    ///
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`
    ///
    /// Speech-to-Text supports three locations: `global`, `us` (US North America),
    /// and `eu` (Europe). If you are calling the `speech.googleapis.com`
    /// endpoint, use the `global` location. To specify a region, use a
    /// [regional endpoint](/speech-to-text/docs/endpoints) with matching `us` or
    /// `eu` location value.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod adaptation_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service that implements Google Cloud Speech Adaptation API."]
    #[derive(Debug, Clone)]
    pub struct AdaptationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdaptationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AdaptationClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AdaptationClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Create a set of phrase hints. Each item in the set can be a single word or"]
        #[doc = " a multi-word phrase. The items in the PhraseSet are favored by the"]
        #[doc = " recognition model when you send a call that includes the PhraseSet."]
        pub async fn create_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePhraseSetRequest>,
        ) -> Result<tonic::Response<super::PhraseSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/CreatePhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a phrase set."]
        pub async fn get_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhraseSetRequest>,
        ) -> Result<tonic::Response<super::PhraseSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/GetPhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List phrase sets."]
        pub async fn list_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhraseSetRequest>,
        ) -> Result<tonic::Response<super::ListPhraseSetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/ListPhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a phrase set."]
        pub async fn update_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePhraseSetRequest>,
        ) -> Result<tonic::Response<super::PhraseSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/UpdatePhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a phrase set."]
        pub async fn delete_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhraseSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/DeletePhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a custom class."]
        pub async fn create_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomClassRequest>,
        ) -> Result<tonic::Response<super::CustomClass>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/CreateCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a custom class."]
        pub async fn get_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomClassRequest>,
        ) -> Result<tonic::Response<super::CustomClass>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/GetCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List custom classes."]
        pub async fn list_custom_classes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomClassesRequest>,
        ) -> Result<tonic::Response<super::ListCustomClassesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/ListCustomClasses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a custom class."]
        pub async fn update_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomClassRequest>,
        ) -> Result<tonic::Response<super::CustomClass>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/UpdateCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a custom class."]
        pub async fn delete_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomClassRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.speech.v1p1beta1.Adaptation/DeleteCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}