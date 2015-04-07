// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *books* crate version *0.1.2+20150309*, where *20150309* is the exact revision of the *books:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.2*.
//! 
//! Everything else about the *books* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/books/docs/v1/getting_started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/books1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Books.html) ... 
//! 
//! * bookshelves
//!  * [*get*](struct.BookshelveGetCall.html), [*list*](struct.BookshelveListCall.html) and [*volumes list*](struct.BookshelveVolumeListCall.html)
//! * cloudloading
//!  * [*add book*](struct.CloudloadingAddBookCall.html), [*delete book*](struct.CloudloadingDeleteBookCall.html) and [*update book*](struct.CloudloadingUpdateBookCall.html)
//! * dictionary
//!  * [*list offline metadata*](struct.DictionaryListOfflineMetadataCall.html)
//! * layers
//!  * [*annotation data get*](struct.LayerAnnotationDataGetCall.html), [*annotation data list*](struct.LayerAnnotationDataListCall.html), [*get*](struct.LayerGetCall.html), [*list*](struct.LayerListCall.html), [*volume annotations get*](struct.LayerVolumeAnnotationGetCall.html) and [*volume annotations list*](struct.LayerVolumeAnnotationListCall.html)
//! * myconfig
//!  * [*get user settings*](struct.MyconfigGetUserSettingCall.html), [*release download access*](struct.MyconfigReleaseDownloadAccesCall.html), [*request access*](struct.MyconfigRequestAccesCall.html), [*sync volume licenses*](struct.MyconfigSyncVolumeLicenseCall.html) and [*update user settings*](struct.MyconfigUpdateUserSettingCall.html)
//! * mylibrary
//!  * [*annotations delete*](struct.MylibraryAnnotationDeleteCall.html), [*annotations insert*](struct.MylibraryAnnotationInsertCall.html), [*annotations list*](struct.MylibraryAnnotationListCall.html), [*annotations summary*](struct.MylibraryAnnotationSummaryCall.html), [*annotations update*](struct.MylibraryAnnotationUpdateCall.html), [*bookshelves add volume*](struct.MylibraryBookshelveAddVolumeCall.html), [*bookshelves clear volumes*](struct.MylibraryBookshelveClearVolumeCall.html), [*bookshelves get*](struct.MylibraryBookshelveGetCall.html), [*bookshelves list*](struct.MylibraryBookshelveListCall.html), [*bookshelves move volume*](struct.MylibraryBookshelveMoveVolumeCall.html), [*bookshelves remove volume*](struct.MylibraryBookshelveRemoveVolumeCall.html), [*bookshelves volumes list*](struct.MylibraryBookshelveVolumeListCall.html), [*readingpositions get*](struct.MylibraryReadingpositionGetCall.html) and [*readingpositions set position*](struct.MylibraryReadingpositionSetPositionCall.html)
//! * onboarding
//!  * [*list categories*](struct.OnboardingListCategoryCall.html) and [*list category volumes*](struct.OnboardingListCategoryVolumeCall.html)
//! * promooffer
//!  * [*accept*](struct.PromoofferAcceptCall.html), [*dismiss*](struct.PromoofferDismisCall.html) and [*get*](struct.PromoofferGetCall.html)
//! * [volumes](struct.Volume.html)
//!  * [*associated list*](struct.VolumeAssociatedListCall.html), [*get*](struct.VolumeGetCall.html), [*list*](struct.VolumeListCall.html), [*mybooks list*](struct.VolumeMybookListCall.html), [*recommended list*](struct.VolumeRecommendedListCall.html), [*recommended rate*](struct.VolumeRecommendedRateCall.html) and [*useruploaded list*](struct.VolumeUseruploadedListCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Books.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.volumes().useruploaded_list(...).doit()
//! let r = hub.myconfig().sync_volume_licenses(...).doit()
//! let r = hub.volumes().list(...).doit()
//! let r = hub.volumes().associated_list(...).doit()
//! let r = hub.bookshelves().volumes_list(...).doit()
//! let r = hub.volumes().recommended_list(...).doit()
//! let r = hub.mylibrary().bookshelves_volumes_list(...).doit()
//! let r = hub.volumes().mybooks_list(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-books1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_books1 as books1;
//! use books1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use books1::Books;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Books::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.volumes().list("q")
//!              .start_index(11)
//!              .source("dolores")
//!              .show_preorders(false)
//!              .projection("sadipscing")
//!              .print_type("aliquyam")
//!              .partner("ea")
//!              .order_by("no")
//!              .max_results(80)
//!              .library_restrict("justo")
//!              .lang_restrict("et")
//!              .filter("et")
//!              .download("diam")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!         Error::MissingToken => println!("OAuth2: Missing Token"),
//!         Error::Cancelled => println!("Operation canceled by user"),
//!         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     },
//!     Ok(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(std_misc)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin, slice_patterns)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Manage your books
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/books",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Books related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// use books1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().list("q")
///              .start_index(96)
///              .source("et")
///              .show_preorders(false)
///              .projection("aliquyam")
///              .print_type("sea")
///              .partner("Lorem")
///              .order_by("eos")
///              .max_results(20)
///              .library_restrict("sadipscing")
///              .lang_restrict("dolor")
///              .filter("eirmod")
///              .download("elitr")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
///         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///         Error::MissingToken => println!("OAuth2: Missing Token"),
///         Error::Cancelled => println!("Operation canceled by user"),
///         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     },
///     Ok(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Books<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Books<C, NC, A> {}

impl<'a, C, NC, A> Books<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Books<C, NC, A> {
        Books {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.2".to_string(),
            _m: PhantomData
        }
    }

    pub fn bookshelves(&'a self) -> BookshelveMethods<'a, C, NC, A> {
        BookshelveMethods { hub: &self }
    }
    pub fn cloudloading(&'a self) -> CloudloadingMethods<'a, C, NC, A> {
        CloudloadingMethods { hub: &self }
    }
    pub fn dictionary(&'a self) -> DictionaryMethods<'a, C, NC, A> {
        DictionaryMethods { hub: &self }
    }
    pub fn layers(&'a self) -> LayerMethods<'a, C, NC, A> {
        LayerMethods { hub: &self }
    }
    pub fn myconfig(&'a self) -> MyconfigMethods<'a, C, NC, A> {
        MyconfigMethods { hub: &self }
    }
    pub fn mylibrary(&'a self) -> MylibraryMethods<'a, C, NC, A> {
        MylibraryMethods { hub: &self }
    }
    pub fn onboarding(&'a self) -> OnboardingMethods<'a, C, NC, A> {
        OnboardingMethods { hub: &self }
    }
    pub fn promooffer(&'a self) -> PromoofferMethods<'a, C, NC, A> {
        PromoofferMethods { hub: &self }
    }
    pub fn volumes(&'a self) -> VolumeMethods<'a, C, NC, A> {
        VolumeMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.2`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list categories onboarding](struct.OnboardingListCategoryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Category {
    /// A list of onboarding categories.
    pub items: Vec<CategoryItems>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for Category {}


/// A list of onboarding categories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct CategoryItems {
    /// no description provided
    #[serde(alias="badgeUrl")]
    pub badge_url: String,
    /// no description provided
    #[serde(alias="categoryId")]
    pub category_id: String,
    /// no description provided
    pub name: String,
}

impl NestedType for CategoryItems {}
impl Part for CategoryItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ConcurrentAccessRestriction {
    /// Client nonce for verification. Download access and client-validation only.
    pub nonce: String,
    /// Resource type.
    pub kind: String,
    /// Whether this volume has any concurrent access restrictions.
    pub restricted: bool,
    /// Identifies the volume for which this entry applies.
    #[serde(alias="volumeId")]
    pub volume_id: String,
    /// The maximum number of concurrent access licenses for this volume.
    #[serde(alias="maxConcurrentDevices")]
    pub max_concurrent_devices: i32,
    /// Whether access is granted for this (user, device, volume).
    #[serde(alias="deviceAllowed")]
    pub device_allowed: bool,
    /// Client app identifier for verification. Download access and client-validation only.
    pub source: String,
    /// Time in seconds for license auto-expiration.
    #[serde(alias="timeWindowSeconds")]
    pub time_window_seconds: i32,
    /// Response signature.
    pub signature: String,
    /// Error/warning reason code.
    #[serde(alias="reasonCode")]
    pub reason_code: String,
    /// Error/warning message.
    pub message: String,
}

impl Part for ConcurrentAccessRestriction {}


/// Physical dimensions of this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeVolumeInfoDimensions {
    /// Width of this volume (in cm).
    pub width: String,
    /// Height or length of this volume (in cm).
    pub height: String,
    /// Thickness of this volume (in cm).
    pub thickness: String,
}

impl NestedType for VolumeVolumeInfoDimensions {}
impl Part for VolumeVolumeInfoDimensions {}


/// General volume information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeVolumeInfo {
    /// Volume subtitle. (In LITE projection.)
    pub subtitle: String,
    /// A synopsis of the volume. The text of the description is formatted in HTML and includes simple formatting elements, such as b, i, and br tags. (In LITE projection.)
    pub description: String,
    /// Total number of pages as per publisher metadata.
    #[serde(alias="pageCount")]
    pub page_count: i32,
    /// A list of image links for all the sizes that are available. (In LITE projection.)
    #[serde(alias="imageLinks")]
    pub image_links: VolumeVolumeInfoImageLinks,
    /// The number of review ratings for this volume.
    #[serde(alias="ratingsCount")]
    pub ratings_count: i32,
    /// The main category to which this volume belongs. It will be the category from the categories list returned below that has the highest weight.
    #[serde(alias="mainCategory")]
    pub main_category: String,
    /// The names of the authors and/or editors for this volume. (In LITE projection)
    pub authors: Vec<String>,
    /// A list of subject categories, such as "Fiction", "Suspense", etc.
    pub categories: Vec<String>,
    /// Publisher of this volume. (In LITE projection.)
    pub publisher: String,
    /// Physical dimensions of this volume.
    pub dimensions: VolumeVolumeInfoDimensions,
    /// Best language for this volume (based on content). It is the two-letter ISO 639-1 code such as 'fr', 'en', etc.
    pub language: String,
    /// URL to preview this volume on the Google Books site.
    #[serde(alias="previewLink")]
    pub preview_link: String,
    /// The reading modes available for this volume.
    #[serde(alias="readingModes")]
    pub reading_modes: String,
    /// Date of publication. (In LITE projection.)
    #[serde(alias="publishedDate")]
    pub published_date: String,
    /// Type of publication of this volume. Possible values are BOOK or MAGAZINE.
    #[serde(alias="printType")]
    pub print_type: String,
    /// Total number of sample pages as per publisher metadata.
    #[serde(alias="samplePageCount")]
    pub sample_page_count: i32,
    /// An identifier for the version of the volume content (text & images). (In LITE projection)
    #[serde(alias="contentVersion")]
    pub content_version: String,
    /// Total number of printed pages in generated pdf representation.
    #[serde(alias="printedPageCount")]
    pub printed_page_count: i32,
    /// Industry standard identifiers for this volume.
    #[serde(alias="industryIdentifiers")]
    pub industry_identifiers: Vec<VolumeVolumeInfoIndustryIdentifiers>,
    /// Volume title. (In LITE projection.)
    pub title: String,
    /// The mean review rating for this volume. (min = 1.0, max = 5.0)
    #[serde(alias="averageRating")]
    pub average_rating: f64,
    /// URL to view information about this volume on the Google Books site. (In LITE projection)
    #[serde(alias="infoLink")]
    pub info_link: String,
    /// Canonical URL for a volume. (In LITE projection.)
    #[serde(alias="canonicalVolumeLink")]
    pub canonical_volume_link: String,
}

impl NestedType for VolumeVolumeInfo {}
impl Part for VolumeVolumeInfo {}


/// Selection ranges sent from the client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationClientVersionRanges {
    /// Range in image CFI format for this annotation sent by client.
    #[serde(alias="imageCfiRange")]
    pub image_cfi_range: BooksAnnotationsRange,
    /// Range in GB text format for this annotation sent by client.
    #[serde(alias="gbTextRange")]
    pub gb_text_range: BooksAnnotationsRange,
    /// Content version the client sent in.
    #[serde(alias="contentVersion")]
    pub content_version: String,
    /// Range in CFI format for this annotation sent by client.
    #[serde(alias="cfiRange")]
    pub cfi_range: BooksAnnotationsRange,
    /// Range in GB image format for this annotation sent by client.
    #[serde(alias="gbImageRange")]
    pub gb_image_range: BooksAnnotationsRange,
}

impl NestedType for AnnotationClientVersionRanges {}
impl Part for AnnotationClientVersionRanges {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [request access myconfig](struct.MyconfigRequestAccesCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RequestAccess {
    /// A download access response.
    #[serde(alias="downloadAccess")]
    pub download_access: DownloadAccessRestriction,
    /// Resource type.
    pub kind: String,
    /// A concurrent access response.
    #[serde(alias="concurrentAccess")]
    pub concurrent_access: ConcurrentAccessRestriction,
}

impl ResponseResult for RequestAccess {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct OffersItemsItems {
    /// no description provided
    pub description: String,
    /// no description provided
    pub author: String,
    /// no description provided
    pub title: String,
    /// no description provided
    #[serde(alias="coverUrl")]
    pub cover_url: String,
    /// no description provided
    #[serde(alias="volumeId")]
    pub volume_id: String,
    /// no description provided
    #[serde(alias="canonicalVolumeLink")]
    pub canonical_volume_link: String,
}

impl NestedType for OffersItemsItems {}
impl Part for OffersItemsItems {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations list mylibrary](struct.MylibraryAnnotationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Annotations {
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// A list of annotations.
    pub items: Vec<Annotation>,
    /// Resource type.
    pub kind: String,
    /// Total number of annotations found. This may be greater than the number of notes returned in this response if results have been paginated.
    #[serde(alias="totalItems")]
    pub total_items: i32,
}

impl ResponseResult for Annotations {}


/// Offers available for this volume (sales and rentals).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSaleInfoOffers {
    /// The rental duration (for rental offers only).
    #[serde(alias="rentalDuration")]
    pub rental_duration: VolumeSaleInfoOffersRentalDuration,
    /// Offer retail (=discounted) price in Micros
    #[serde(alias="retailPrice")]
    pub retail_price: VolumeSaleInfoOffersRetailPrice,
    /// Offer list (=undiscounted) price in Micros.
    #[serde(alias="listPrice")]
    pub list_price: VolumeSaleInfoOffersListPrice,
    /// The finsky offer type (e.g., PURCHASE=0 RENTAL=3)
    #[serde(alias="finskyOfferType")]
    pub finsky_offer_type: i32,
}

impl NestedType for VolumeSaleInfoOffers {}
impl Part for VolumeSaleInfoOffers {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list offline metadata dictionary](struct.DictionaryListOfflineMetadataCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Metadata {
    /// A list of offline dictionary metadata.
    pub items: Vec<MetadataItems>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for Metadata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [useruploaded list volumes](struct.VolumeUseruploadedListCall.html) (none)
/// * [recommended rate volumes](struct.VolumeRecommendedRateCall.html) (none)
/// * [list volumes](struct.VolumeListCall.html) (none)
/// * [associated list volumes](struct.VolumeAssociatedListCall.html) (none)
/// * [get volumes](struct.VolumeGetCall.html) (response)
/// * [recommended list volumes](struct.VolumeRecommendedListCall.html) (none)
/// * [mybooks list volumes](struct.VolumeMybookListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Volume {
    /// Resource type for a volume. (In LITE projection.)
    pub kind: String,
    /// Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.).
    #[serde(alias="accessInfo")]
    pub access_info: VolumeAccessInfo,
    /// Search result information related to this volume.
    #[serde(alias="searchInfo")]
    pub search_info: VolumeSearchInfo,
    /// Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries).
    #[serde(alias="saleInfo")]
    pub sale_info: VolumeSaleInfo,
    /// Opaque identifier for a specific version of a volume resource. (In LITE projection)
    pub etag: String,
    /// What layers exist in this volume and high level information about them.
    #[serde(alias="layerInfo")]
    pub layer_info: VolumeLayerInfo,
    /// General volume information.
    #[serde(alias="volumeInfo")]
    pub volume_info: VolumeVolumeInfo,
    /// Recommendation related information for this volume.
    #[serde(alias="recommendedInfo")]
    pub recommended_info: VolumeRecommendedInfo,
    /// Unique identifier for a volume. (In LITE projection.)
    pub id: String,
    /// URL to this resource. (In LITE projection.)
    #[serde(alias="selfLink")]
    pub self_link: String,
    /// User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)
    #[serde(alias="userInfo")]
    pub user_info: VolumeUserInfo,
}

impl Resource for Volume {}
impl ResponseResult for Volume {}


/// What layers exist in this volume and high level information about them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeLayerInfo {
    /// A layer should appear here if and only if the layer exists for this book.
    pub layers: Vec<VolumeLayerInfoLayers>,
}

impl NestedType for VolumeLayerInfo {}
impl Part for VolumeLayerInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get user settings myconfig](struct.MyconfigGetUserSettingCall.html) (response)
/// * [update user settings myconfig](struct.MyconfigUpdateUserSettingCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Usersettings {
    /// Resource type.
    pub kind: Option<String>,
    /// User settings in sub-objects, each for different purposes.
    #[serde(alias="notesExport")]
    pub notes_export: Option<UsersettingsNotesExport>,
}

impl RequestValue for Usersettings {}
impl ResponseResult for Usersettings {}


/// Recommendation related information for this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeRecommendedInfo {
    /// A text explaining why this volume is recommended.
    pub explanation: String,
}

impl NestedType for VolumeRecommendedInfo {}
impl Part for VolumeRecommendedInfo {}


/// A list of offline dictionary metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct MetadataItems {
    /// no description provided
    pub encrypted_key: String,
    /// no description provided
    pub version: String,
    /// no description provided
    pub download_url: String,
    /// no description provided
    pub language: String,
    /// no description provided
    pub size: String,
}

impl NestedType for MetadataItems {}
impl Part for MetadataItems {}


/// Author of this review.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReviewAuthor {
    /// Name of this person.
    #[serde(alias="displayName")]
    pub display_name: String,
}

impl NestedType for ReviewAuthor {}
impl Part for ReviewAuthor {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get bookshelves](struct.BookshelveGetCall.html) (response)
/// * [bookshelves get mylibrary](struct.MylibraryBookshelveGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Bookshelf {
    /// Resource type for bookshelf metadata.
    pub kind: String,
    /// Description of this bookshelf.
    pub description: String,
    /// Title of this bookshelf.
    pub title: String,
    /// Number of volumes in this bookshelf.
    #[serde(alias="volumeCount")]
    pub volume_count: i32,
    /// Created time for this bookshelf (formatted UTC timestamp with millisecond resolution).
    pub created: String,
    /// Last modified time of this bookshelf (formatted UTC timestamp with millisecond resolution).
    pub updated: String,
    /// Whether this bookshelf is PUBLIC or PRIVATE.
    pub access: String,
    /// Last time a volume was added or removed from this bookshelf (formatted UTC timestamp with millisecond resolution).
    #[serde(alias="volumesLastUpdated")]
    pub volumes_last_updated: String,
    /// Id of this bookshelf, only unique by user.
    pub id: i32,
    /// URL to this resource.
    #[serde(alias="selfLink")]
    pub self_link: String,
}

impl ResponseResult for Bookshelf {}


/// User settings in sub-objects, each for different purposes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotesExport {
    /// no description provided
    #[serde(alias="isEnabled")]
    pub is_enabled: bool,
    /// no description provided
    #[serde(alias="folderName")]
    pub folder_name: String,
}

impl NestedType for UsersettingsNotesExport {}
impl Part for UsersettingsNotesExport {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations insert mylibrary](struct.MylibraryAnnotationInsertCall.html) (request|response)
/// * [annotations update mylibrary](struct.MylibraryAnnotationUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotation {
    /// Timestamp for the last time this annotation was modified.
    pub updated: Option<String>,
    /// Indicates that this annotation is deleted.
    pub deleted: Option<bool>,
    /// Selection ranges for the most recent content version.
    #[serde(alias="currentVersionRanges")]
    pub current_version_ranges: Option<AnnotationCurrentVersionRanges>,
    /// Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
    #[serde(alias="afterSelectedText")]
    pub after_selected_text: Option<String>,
    /// The volume that this annotation belongs to.
    #[serde(alias="volumeId")]
    pub volume_id: Option<String>,
    /// Excerpt from the volume.
    #[serde(alias="selectedText")]
    pub selected_text: Option<String>,
    /// User-created data for this annotation.
    pub data: Option<String>,
    /// Id of this annotation, in the form of a GUID.
    pub id: Option<String>,
    /// Resource type.
    pub kind: Option<String>,
    /// Timestamp for the created time of this annotation.
    pub created: Option<String>,
    /// Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
    #[serde(alias="beforeSelectedText")]
    pub before_selected_text: Option<String>,
    /// Selection ranges sent from the client.
    #[serde(alias="clientVersionRanges")]
    pub client_version_ranges: Option<AnnotationClientVersionRanges>,
    /// Pages that this annotation spans.
    #[serde(alias="pageIds")]
    pub page_ids: Option<Vec<String>>,
    /// The layer this annotation is for.
    #[serde(alias="layerId")]
    pub layer_id: Option<String>,
    /// The highlight style for this annotation.
    #[serde(alias="highlightStyle")]
    pub highlight_style: Option<String>,
    /// no description provided
    #[serde(alias="layerSummary")]
    pub layer_summary: Option<AnnotationLayerSummary>,
    /// URL to this resource.
    #[serde(alias="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for Annotation {}
impl ResponseResult for Annotation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Review {
    /// Star rating for this review. Possible values are ONE, TWO, THREE, FOUR, FIVE or NOT_RATED.
    pub rating: String,
    /// Resource type for a review.
    pub kind: String,
    /// Title for this review.
    pub title: String,
    /// Author of this review.
    pub author: ReviewAuthor,
    /// Volume that this review is for.
    #[serde(alias="volumeId")]
    pub volume_id: String,
    /// Review text.
    pub content: String,
    /// Information regarding the source of this review, when the review is not from a Google Books user.
    pub source: ReviewSource,
    /// Date of this review.
    pub date: String,
    /// Source type for this review. Possible values are EDITORIAL, WEB_USER or GOOGLE_USER.
    #[serde(alias="type")]
    pub type_: String,
    /// URL for the full review text, for reviews gathered from the web.
    #[serde(alias="fullTextUrl")]
    pub full_text_url: String,
}

impl Part for Review {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DownloadAccessRestriction {
    /// Client nonce for verification. Download access and client-validation only.
    pub nonce: String,
    /// Resource type.
    pub kind: String,
    /// If deviceAllowed, whether access was just acquired with this request.
    #[serde(alias="justAcquired")]
    pub just_acquired: bool,
    /// If restricted, the maximum number of content download licenses for this volume.
    #[serde(alias="maxDownloadDevices")]
    pub max_download_devices: i32,
    /// If restricted, the number of content download licenses already acquired (including the requesting client, if licensed).
    #[serde(alias="downloadsAcquired")]
    pub downloads_acquired: i32,
    /// Identifies the volume for which this entry applies.
    #[serde(alias="volumeId")]
    pub volume_id: String,
    /// If restricted, whether access is granted for this (user, device, volume).
    #[serde(alias="deviceAllowed")]
    pub device_allowed: bool,
    /// Client app identifier for verification. Download access and client-validation only.
    pub source: String,
    /// Response signature.
    pub signature: String,
    /// Error/warning reason code. Additional codes may be added in the future. 0 OK 100 ACCESS_DENIED_PUBLISHER_LIMIT 101 ACCESS_DENIED_LIMIT 200 WARNING_USED_LAST_ACCESS
    #[serde(alias="reasonCode")]
    pub reason_code: String,
    /// Error/warning message.
    pub message: String,
    /// Whether this volume has any download access restrictions.
    pub restricted: bool,
}

impl Part for DownloadAccessRestriction {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [release download access myconfig](struct.MyconfigReleaseDownloadAccesCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DownloadAccesses {
    /// A list of download access responses.
    #[serde(alias="downloadAccessList")]
    pub download_access_list: Vec<DownloadAccessRestriction>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for DownloadAccesses {}


/// Information regarding the source of this review, when the review is not from a Google Books user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReviewSource {
    /// Extra text about the source of the review.
    #[serde(alias="extraDescription")]
    pub extra_description: String,
    /// URL of the source of the review.
    pub url: String,
    /// Name of the source.
    pub description: String,
}

impl NestedType for ReviewSource {}
impl Part for ReviewSource {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [readingpositions get mylibrary](struct.MylibraryReadingpositionGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReadingPosition {
    /// Position in a PDF file.
    #[serde(alias="pdfPosition")]
    pub pdf_position: String,
    /// Resource type for a reading position.
    pub kind: String,
    /// Position in a volume for image-based content.
    #[serde(alias="gbImagePosition")]
    pub gb_image_position: String,
    /// Position in a volume for text-based content.
    #[serde(alias="gbTextPosition")]
    pub gb_text_position: String,
    /// Position in an EPUB as a CFI.
    #[serde(alias="epubCfiPosition")]
    pub epub_cfi_position: String,
    /// Timestamp when this reading position was last updated (formatted UTC timestamp with millisecond resolution).
    pub updated: String,
    /// Volume id associated with this reading position.
    #[serde(alias="volumeId")]
    pub volume_id: String,
}

impl ResponseResult for ReadingPosition {}


/// Offer retail (=discounted) price in Micros
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSaleInfoOffersRetailPrice {
    /// no description provided
    #[serde(alias="currencyCode")]
    pub currency_code: String,
    /// no description provided
    #[serde(alias="amountInMicros")]
    pub amount_in_micros: f64,
}

impl NestedType for VolumeSaleInfoOffersRetailPrice {}
impl Part for VolumeSaleInfoOffersRetailPrice {}


/// Search result information related to this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSearchInfo {
    /// A text snippet containing the search query.
    #[serde(alias="textSnippet")]
    pub text_snippet: String,
}

impl NestedType for VolumeSearchInfo {}
impl Part for VolumeSearchInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AnnotationsSummaryLayers {
    /// no description provided
    #[serde(alias="limitType")]
    pub limit_type: String,
    /// no description provided
    #[serde(alias="remainingCharacterCount")]
    pub remaining_character_count: i32,
    /// no description provided
    pub updated: String,
    /// no description provided
    #[serde(alias="allowedCharacterCount")]
    pub allowed_character_count: i32,
    /// no description provided
    #[serde(alias="layerId")]
    pub layer_id: String,
}

impl NestedType for AnnotationsSummaryLayers {}
impl Part for AnnotationsSummaryLayers {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation data get layers](struct.LayerAnnotationDataGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Annotationdata {
    /// The type of annotation this data is for.
    #[serde(alias="annotationType")]
    pub annotation_type: String,
    /// Resource Type
    pub kind: String,
    /// Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format).
    pub updated: String,
    /// The volume id for this data. *
    #[serde(alias="volumeId")]
    pub volume_id: String,
    /// Base64 encoded data for this annotation data.
    pub encoded_data: String,
    /// The Layer id for this data. *
    #[serde(alias="layerId")]
    pub layer_id: String,
    /// no description provided
    pub data: String,
    /// Unique id for this annotation data.
    pub id: String,
    /// URL for this resource. *
    #[serde(alias="selfLink")]
    pub self_link: String,
}

impl ResponseResult for Annotationdata {}


/// A list of image links for all the sizes that are available. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeVolumeInfoImageLinks {
    /// Image link for large size (width of ~800 pixels). (In LITE projection)
    pub large: String,
    /// Image link for extra large size (width of ~1280 pixels). (In LITE projection)
    #[serde(alias="extraLarge")]
    pub extra_large: String,
    /// Image link for medium size (width of ~575 pixels). (In LITE projection)
    pub medium: String,
    /// Image link for small thumbnail size (width of ~80 pixels). (In LITE projection)
    #[serde(alias="smallThumbnail")]
    pub small_thumbnail: String,
    /// Image link for small size (width of ~300 pixels). (In LITE projection)
    pub small: String,
    /// Image link for thumbnail size (width of ~128 pixels). (In LITE projection)
    pub thumbnail: String,
}

impl NestedType for VolumeVolumeInfoImageLinks {}
impl Part for VolumeVolumeInfoImageLinks {}


/// The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSaleInfoRetailPrice {
    /// Amount in the currency listed below. (In LITE projection.)
    pub amount: f64,
    /// An ISO 4217, three-letter currency code. (In LITE projection.)
    #[serde(alias="currencyCode")]
    pub currency_code: String,
}

impl NestedType for VolumeSaleInfoRetailPrice {}
impl Part for VolumeSaleInfoRetailPrice {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get layers](struct.LayerGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Layersummary {
    /// Resource Type
    pub kind: String,
    /// The number of annotations for this layer.
    #[serde(alias="annotationCount")]
    pub annotation_count: i32,
    /// The number of data items for this layer.
    #[serde(alias="dataCount")]
    pub data_count: i32,
    /// The list of annotation types contained for this layer.
    #[serde(alias="annotationTypes")]
    pub annotation_types: Vec<String>,
    /// Timestamp for the last time an item in this layer was updated. (RFC 3339 UTC date-time format).
    pub updated: String,
    /// The volume id this resource is for.
    #[serde(alias="volumeId")]
    pub volume_id: String,
    /// Link to get data for this annotation.
    #[serde(alias="annotationsDataLink")]
    pub annotations_data_link: String,
    /// The link to get the annotations for this layer.
    #[serde(alias="annotationsLink")]
    pub annotations_link: String,
    /// The content version this resource is for.
    #[serde(alias="contentVersion")]
    pub content_version: String,
    /// The layer id for this summary.
    #[serde(alias="layerId")]
    pub layer_id: String,
    /// The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately.
    #[serde(alias="volumeAnnotationsVersion")]
    pub volume_annotations_version: String,
    /// Unique id of this layer summary.
    pub id: String,
    /// URL to this resource.
    #[serde(alias="selfLink")]
    pub self_link: String,
}

impl ResponseResult for Layersummary {}


/// Offer list (=undiscounted) price in Micros.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSaleInfoOffersListPrice {
    /// no description provided
    #[serde(alias="currencyCode")]
    pub currency_code: String,
    /// no description provided
    #[serde(alias="amountInMicros")]
    pub amount_in_micros: f64,
}

impl NestedType for VolumeSaleInfoOffersListPrice {}
impl Part for VolumeSaleInfoOffersListPrice {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [volume annotations get layers](struct.LayerVolumeAnnotationGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Volumeannotation {
    /// The type of annotation this is.
    #[serde(alias="annotationType")]
    pub annotation_type: String,
    /// Resource Type
    pub kind: String,
    /// Indicates that this annotation is deleted.
    pub deleted: bool,
    /// The content ranges to identify the selected text.
    #[serde(alias="contentRanges")]
    pub content_ranges: VolumeannotationContentRanges,
    /// Timestamp for the last time this anntoation was updated. (RFC 3339 UTC date-time format).
    pub updated: String,
    /// The Volume this annotation is for.
    #[serde(alias="volumeId")]
    pub volume_id: String,
    /// The annotation data id for this volume annotation.
    #[serde(alias="annotationDataId")]
    pub annotation_data_id: String,
    /// Link to get data for this annotation.
    #[serde(alias="annotationDataLink")]
    pub annotation_data_link: String,
    /// Pages the annotation spans.
    #[serde(alias="pageIds")]
    pub page_ids: Vec<String>,
    /// The Layer this annotation is for.
    #[serde(alias="layerId")]
    pub layer_id: String,
    /// Excerpt from the volume.
    #[serde(alias="selectedText")]
    pub selected_text: String,
    /// Data for this annotation.
    pub data: String,
    /// Unique id of this volume annotation.
    pub id: String,
    /// URL to this resource.
    #[serde(alias="selfLink")]
    pub self_link: String,
}

impl ResponseResult for Volumeannotation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [volume annotations list layers](struct.LayerVolumeAnnotationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Volumeannotations {
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// A list of volume annotations.
    pub items: Vec<Volumeannotation>,
    /// Resource type
    pub kind: String,
    /// The version string for all of the volume annotations in this layer (not just the ones in this response). Note: the version string doesn't apply to the annotation data, just the information in this response (e.g. the location of annotations in the book).
    pub version: String,
    /// The total number of volume annotations found.
    #[serde(alias="totalItems")]
    pub total_items: i32,
}

impl ResponseResult for Volumeannotations {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation data list layers](struct.LayerAnnotationDataListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Annotationsdata {
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// A list of Annotation Data.
    pub items: Vec<Annotationdata>,
    /// Resource type
    pub kind: String,
    /// The total number of volume annotations found.
    #[serde(alias="totalItems")]
    pub total_items: i32,
}

impl ResponseResult for Annotationsdata {}


/// Industry standard identifiers for this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeVolumeInfoIndustryIdentifiers {
    /// Industry specific volume identifier.
    pub identifier: String,
    /// Identifier type. Possible values are ISBN_10, ISBN_13, ISSN and OTHER.
    #[serde(alias="type")]
    pub type_: String,
}

impl NestedType for VolumeVolumeInfoIndustryIdentifiers {}
impl Part for VolumeVolumeInfoIndustryIdentifiers {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationLayerSummary {
    /// Type of limitation on this layer. "limited" or "unlimited" for the "copy" layer.
    #[serde(alias="limitType")]
    pub limit_type: String,
    /// Remaining allowed characters on this layer, especially for the "copy" layer.
    #[serde(alias="remainingCharacterCount")]
    pub remaining_character_count: i32,
    /// Maximum allowed characters on this layer, especially for the "copy" layer.
    #[serde(alias="allowedCharacterCount")]
    pub allowed_character_count: i32,
}

impl NestedType for AnnotationLayerSummary {}
impl Part for AnnotationLayerSummary {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get promooffer](struct.PromoofferGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Offers {
    /// A list of offers.
    pub items: Vec<OffersItems>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for Offers {}


/// Information about epub content. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeAccessInfoEpub {
    /// Is a flowing text epub available either as public domain or for purchase. (In LITE projection.)
    #[serde(alias="isAvailable")]
    pub is_available: bool,
    /// URL to download epub. (In LITE projection.)
    #[serde(alias="downloadLink")]
    pub download_link: String,
    /// URL to retrieve ACS token for epub download. (In LITE projection.)
    #[serde(alias="acsTokenLink")]
    pub acs_token_link: String,
}

impl NestedType for VolumeAccessInfoEpub {}
impl Part for VolumeAccessInfoEpub {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list category volumes onboarding](struct.OnboardingListCategoryVolumeCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Volume2 {
    /// no description provided
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// A list of volumes.
    pub items: Vec<Volume>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for Volume2 {}


/// Information about pdf content. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeAccessInfoPdf {
    /// Is a scanned image pdf available either as public domain or for purchase. (In LITE projection.)
    #[serde(alias="isAvailable")]
    pub is_available: bool,
    /// URL to download pdf. (In LITE projection.)
    #[serde(alias="downloadLink")]
    pub download_link: String,
    /// URL to retrieve ACS token for pdf download. (In LITE projection.)
    #[serde(alias="acsTokenLink")]
    pub acs_token_link: String,
}

impl NestedType for VolumeAccessInfoPdf {}
impl Part for VolumeAccessInfoPdf {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations summary mylibrary](struct.MylibraryAnnotationSummaryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AnnotationsSummary {
    /// no description provided
    pub layers: Vec<AnnotationsSummaryLayers>,
    /// no description provided
    pub kind: String,
}

impl ResponseResult for AnnotationsSummary {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bookshelves list mylibrary](struct.MylibraryBookshelveListCall.html) (response)
/// * [list bookshelves](struct.BookshelveListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Bookshelves {
    /// A list of bookshelves.
    pub items: Vec<Bookshelf>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for Bookshelves {}


/// A layer should appear here if and only if the layer exists for this book.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeLayerInfoLayers {
    /// The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately.
    #[serde(alias="volumeAnnotationsVersion")]
    pub volume_annotations_version: String,
    /// The layer id of this layer (e.g. "geo").
    #[serde(alias="layerId")]
    pub layer_id: String,
}

impl NestedType for VolumeLayerInfoLayers {}
impl Part for VolumeLayerInfoLayers {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list layers](struct.LayerListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Layersummaries {
    /// The total number of layer summaries found.
    #[serde(alias="totalItems")]
    pub total_items: i32,
    /// A list of layer summary items.
    pub items: Vec<Layersummary>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for Layersummaries {}


/// A list of offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct OffersItems {
    /// no description provided
    #[serde(alias="gservicesKey")]
    pub gservices_key: String,
    /// no description provided
    pub items: Vec<OffersItemsItems>,
    /// no description provided
    #[serde(alias="artUrl")]
    pub art_url: String,
    /// no description provided
    pub id: String,
}

impl NestedType for OffersItems {}
impl Part for OffersItems {}


/// Copy/Paste accounting information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeUserInfoCopy {
    /// no description provided
    #[serde(alias="limitType")]
    pub limit_type: String,
    /// no description provided
    #[serde(alias="remainingCharacterCount")]
    pub remaining_character_count: i32,
    /// no description provided
    pub updated: String,
    /// no description provided
    #[serde(alias="allowedCharacterCount")]
    pub allowed_character_count: i32,
}

impl NestedType for VolumeUserInfoCopy {}
impl Part for VolumeUserInfoCopy {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recommended rate volumes](struct.VolumeRecommendedRateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct BooksVolumesRecommendedRateResponse {
    /// no description provided
    pub consistency_token: String,
}

impl ResponseResult for BooksVolumesRecommendedRateResponse {}


/// User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeUserInfo {
    /// Whether or not this volume is currently in "my books."
    #[serde(alias="isInMyBooks")]
    pub is_in_my_books: bool,
    /// Period during this book is/was a valid rental.
    #[serde(alias="rentalPeriod")]
    pub rental_period: VolumeUserInfoRentalPeriod,
    /// Timestamp when this volume was last modified by a user action, such as a reading position update, volume purchase or writing a review. (RFC 3339 UTC date-time format).
    pub updated: String,
    /// no description provided
    #[serde(alias="userUploadedVolumeInfo")]
    pub user_uploaded_volume_info: VolumeUserInfoUserUploadedVolumeInfo,
    /// Whether this book is an active or an expired rental.
    #[serde(alias="rentalState")]
    pub rental_state: String,
    /// Whether or not this volume was purchased by the authenticated user making the request. (In LITE projection.)
    #[serde(alias="isPurchased")]
    pub is_purchased: bool,
    /// The user's current reading position in the volume, if one is available. (In LITE projection.)
    #[serde(alias="readingPosition")]
    pub reading_position: ReadingPosition,
    /// Whether or not this volume was pre-ordered by the authenticated user making the request. (In LITE projection.)
    #[serde(alias="isPreordered")]
    pub is_preordered: bool,
    /// Copy/Paste accounting information.
    pub copy: VolumeUserInfoCopy,
    /// This user's review of this volume, if one exists.
    pub review: Review,
    /// Whether or not this volume was user uploaded.
    #[serde(alias="isUploaded")]
    pub is_uploaded: bool,
}

impl NestedType for VolumeUserInfo {}
impl Part for VolumeUserInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update book cloudloading](struct.CloudloadingUpdateBookCall.html) (request|response)
/// * [add book cloudloading](struct.CloudloadingAddBookCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksCloudloadingResource {
    /// no description provided
    pub author: Option<String>,
    /// no description provided
    #[serde(alias="processingState")]
    pub processing_state: Option<String>,
    /// no description provided
    #[serde(alias="volumeId")]
    pub volume_id: Option<String>,
    /// no description provided
    pub title: Option<String>,
}

impl RequestValue for BooksCloudloadingResource {}
impl ResponseResult for BooksCloudloadingResource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeUserInfoUserUploadedVolumeInfo {
    /// no description provided
    #[serde(alias="processingState")]
    pub processing_state: String,
}

impl NestedType for VolumeUserInfoUserUploadedVolumeInfo {}
impl Part for VolumeUserInfoUserUploadedVolumeInfo {}


/// Suggested retail price. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSaleInfoListPrice {
    /// Amount in the currency listed below. (In LITE projection.)
    pub amount: f64,
    /// An ISO 4217, three-letter currency code. (In LITE projection.)
    #[serde(alias="currencyCode")]
    pub currency_code: String,
}

impl NestedType for VolumeSaleInfoListPrice {}
impl Part for VolumeSaleInfoListPrice {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksAnnotationsRange {
    /// The starting position for the range.
    #[serde(alias="startPosition")]
    pub start_position: String,
    /// The ending position for the range.
    #[serde(alias="endPosition")]
    pub end_position: String,
    /// The offset from the starting position.
    #[serde(alias="startOffset")]
    pub start_offset: String,
    /// The offset from the ending position.
    #[serde(alias="endOffset")]
    pub end_offset: String,
}

impl Part for BooksAnnotationsRange {}


/// Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeAccessInfo {
    /// URL to read this volume on the Google Books site. Link will not allow users to read non-viewable volumes.
    #[serde(alias="webReaderLink")]
    pub web_reader_link: String,
    /// Whether or not this book is public domain in the country listed above.
    #[serde(alias="publicDomain")]
    pub public_domain: bool,
    /// Whether this volume requires that the client explicitly request offline download license rather than have it done automatically when loading the content, if the client supports it.
    #[serde(alias="explicitOfflineLicenseManagement")]
    pub explicit_offline_license_management: bool,
    /// Whether this volume can be embedded in a viewport using the Embedded Viewer API.
    pub embeddable: bool,
    /// Information about a volume's download license access restrictions.
    #[serde(alias="downloadAccess")]
    pub download_access: DownloadAccessRestriction,
    /// The two-letter ISO_3166-1 country code for which this access information is valid. (In LITE projection.)
    pub country: String,
    /// For ordered but not yet processed orders, we give a URL that can be used to go to the appropriate Google Wallet page.
    #[serde(alias="viewOrderUrl")]
    pub view_order_url: String,
    /// Whether text-to-speech is permitted for this volume. Values can be ALLOWED, ALLOWED_FOR_ACCESSIBILITY, or NOT_ALLOWED.
    #[serde(alias="textToSpeechPermission")]
    pub text_to_speech_permission: String,
    /// URL to the Google Drive viewer if this volume is uploaded by the user by selecting the file from Google Drive.
    #[serde(alias="driveImportedContentLink")]
    pub drive_imported_content_link: String,
    /// Information about pdf content. (In LITE projection.)
    pub pdf: VolumeAccessInfoPdf,
    /// Whether quote sharing is allowed for this volume.
    #[serde(alias="quoteSharingAllowed")]
    pub quote_sharing_allowed: bool,
    /// The read access of a volume. Possible values are PARTIAL, ALL_PAGES, NO_PAGES or UNKNOWN. This value depends on the country listed above. A value of PARTIAL means that the publisher has allowed some portion of the volume to be viewed publicly, without purchase. This can apply to eBooks as well as non-eBooks. Public domain books will always have a value of ALL_PAGES.
    pub viewability: String,
    /// Information about epub content. (In LITE projection.)
    pub epub: VolumeAccessInfoEpub,
    /// Combines the access and viewability of this volume into a single status field for this user. Values can be FULL_PURCHASED, FULL_PUBLIC_DOMAIN, SAMPLE or NONE. (In LITE projection.)
    #[serde(alias="accessViewStatus")]
    pub access_view_status: String,
}

impl NestedType for VolumeAccessInfo {}
impl Part for VolumeAccessInfo {}


/// Selection ranges for the most recent content version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationCurrentVersionRanges {
    /// Range in image CFI format for this annotation for version above.
    #[serde(alias="imageCfiRange")]
    pub image_cfi_range: BooksAnnotationsRange,
    /// Range in GB text format for this annotation for version above.
    #[serde(alias="gbTextRange")]
    pub gb_text_range: BooksAnnotationsRange,
    /// Content version applicable to ranges below.
    #[serde(alias="contentVersion")]
    pub content_version: String,
    /// Range in CFI format for this annotation for version above.
    #[serde(alias="cfiRange")]
    pub cfi_range: BooksAnnotationsRange,
    /// Range in GB image format for this annotation for version above.
    #[serde(alias="gbImageRange")]
    pub gb_image_range: BooksAnnotationsRange,
}

impl NestedType for AnnotationCurrentVersionRanges {}
impl Part for AnnotationCurrentVersionRanges {}


/// Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSaleInfo {
    /// The two-letter ISO_3166-1 country code for which this sale information is valid. (In LITE projection.)
    pub country: String,
    /// The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)
    #[serde(alias="retailPrice")]
    pub retail_price: VolumeSaleInfoRetailPrice,
    /// Whether or not this volume is an eBook (can be added to the My eBooks shelf).
    #[serde(alias="isEbook")]
    pub is_ebook: bool,
    /// Offers available for this volume (sales and rentals).
    pub offers: Vec<VolumeSaleInfoOffers>,
    /// Whether or not this book is available for sale or offered for free in the Google eBookstore for the country listed above. Possible values are FOR_SALE, FOR_RENTAL_ONLY, FOR_SALE_AND_RENTAL, FREE, NOT_FOR_SALE, or FOR_PREORDER.
    pub saleability: String,
    /// URL to purchase this volume on the Google Books site. (In LITE projection)
    #[serde(alias="buyLink")]
    pub buy_link: String,
    /// The date on which this book is available for sale.
    #[serde(alias="onSaleDate")]
    pub on_sale_date: String,
    /// Suggested retail price. (In LITE projection.)
    #[serde(alias="listPrice")]
    pub list_price: VolumeSaleInfoListPrice,
}

impl NestedType for VolumeSaleInfo {}
impl Part for VolumeSaleInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [useruploaded list volumes](struct.VolumeUseruploadedListCall.html) (response)
/// * [sync volume licenses myconfig](struct.MyconfigSyncVolumeLicenseCall.html) (response)
/// * [list volumes](struct.VolumeListCall.html) (response)
/// * [associated list volumes](struct.VolumeAssociatedListCall.html) (response)
/// * [volumes list bookshelves](struct.BookshelveVolumeListCall.html) (response)
/// * [recommended list volumes](struct.VolumeRecommendedListCall.html) (response)
/// * [bookshelves volumes list mylibrary](struct.MylibraryBookshelveVolumeListCall.html) (response)
/// * [mybooks list volumes](struct.VolumeMybookListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Volumes {
    /// Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated.
    #[serde(alias="totalItems")]
    pub total_items: i32,
    /// A list of volumes.
    pub items: Vec<Volume>,
    /// Resource type.
    pub kind: String,
}

impl ResponseResult for Volumes {}


/// Period during this book is/was a valid rental.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeUserInfoRentalPeriod {
    /// no description provided
    #[serde(alias="startUtcSec")]
    pub start_utc_sec: String,
    /// no description provided
    #[serde(alias="endUtcSec")]
    pub end_utc_sec: String,
}

impl NestedType for VolumeUserInfoRentalPeriod {}
impl Part for VolumeUserInfoRentalPeriod {}


/// The content ranges to identify the selected text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeannotationContentRanges {
    /// Range in GB image format for this annotation for version above.
    #[serde(alias="gbImageRange")]
    pub gb_image_range: BooksAnnotationsRange,
    /// Range in GB text format for this annotation for version above.
    #[serde(alias="gbTextRange")]
    pub gb_text_range: BooksAnnotationsRange,
    /// Content version applicable to ranges below.
    #[serde(alias="contentVersion")]
    pub content_version: String,
    /// Range in CFI format for this annotation for version above.
    #[serde(alias="cfiRange")]
    pub cfi_range: BooksAnnotationsRange,
}

impl NestedType for VolumeannotationContentRanges {}
impl Part for VolumeannotationContentRanges {}


/// The rental duration (for rental offers only).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VolumeSaleInfoOffersRentalDuration {
    /// no description provided
    pub count: f64,
    /// no description provided
    pub unit: String,
}

impl NestedType for VolumeSaleInfoOffersRentalDuration {}
impl Part for VolumeSaleInfoOffersRentalDuration {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *layer* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotation_data_get(...)`, `annotation_data_list(...)`, `get(...)`, `list(...)`, `volume_annotations_get(...)` and `volume_annotations_list(...)`
/// // to build up your call.
/// let rb = hub.layers();
/// # }
/// ```
pub struct LayerMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for LayerMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> LayerMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationDataId` - The ID of the annotation data to retrieve.
    /// * `contentVersion` - The content version for the volume you are trying to retrieve.
    pub fn annotation_data_get(&self, volume_id: &str, layer_id: &str, annotation_data_id: &str, content_version: &str) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        LayerAnnotationDataGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_data_id: annotation_data_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _allow_web_definitions: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotation.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationId` - The ID of the volume annotation to retrieve.
    pub fn volume_annotations_get(&self, volume_id: &str, layer_id: &str, annotation_id: &str) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> {
        LayerVolumeAnnotationGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the layer summaries for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    pub fn list(&self, volume_id: &str) -> LayerListCall<'a, C, NC, A> {
        LayerListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the layer summary for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    /// * `summaryId` - The ID for the layer to get the summary for.
    pub fn get(&self, volume_id: &str, summary_id: &str) -> LayerGetCall<'a, C, NC, A> {
        LayerGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _summary_id: summary_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotations for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn volume_annotations_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        LayerVolumeAnnotationListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _volume_annotations_version: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _start_position: Default::default(),
            _start_offset: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _end_position: Default::default(),
            _end_offset: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotation data for.
    /// * `layerId` - The ID for the layer to get the annotation data.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn annotation_data_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        LayerAnnotationDataListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _annotation_data_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *volume* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `associated_list(...)`, `get(...)`, `list(...)`, `mybooks_list(...)`, `recommended_list(...)`, `recommended_rate(...)` and `useruploaded_list(...)`
/// // to build up your call.
/// let rb = hub.volumes();
/// # }
/// ```
pub struct VolumeMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for VolumeMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rate a recommended book for the current user.
    /// 
    /// # Arguments
    ///
    /// * `rating` - Rating to be given to the volume.
    /// * `volumeId` - ID of the source volume.
    pub fn recommended_rate(&self, rating: &str, volume_id: &str) -> VolumeRecommendedRateCall<'a, C, NC, A> {
        VolumeRecommendedRateCall {
            hub: self.hub,
            _rating: rating.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books in My Library.
    pub fn mybooks_list(&self) -> VolumeMybookListCall<'a, C, NC, A> {
        VolumeMybookListCall {
            hub: self.hub,
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _acquire_method: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a book search.
    /// 
    /// # Arguments
    ///
    /// * `q` - Full-text search query string.
    pub fn list(&self, q: &str) -> VolumeListCall<'a, C, NC, A> {
        VolumeListCall {
            hub: self.hub,
            _q: q.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _projection: Default::default(),
            _print_type: Default::default(),
            _partner: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _library_restrict: Default::default(),
            _lang_restrict: Default::default(),
            _filter: Default::default(),
            _download: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books uploaded by the current user.
    pub fn useruploaded_list(&self) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        VolumeUseruploadedListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of associated books.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of the source volume.
    pub fn associated_list(&self, volume_id: &str) -> VolumeAssociatedListCall<'a, C, NC, A> {
        VolumeAssociatedListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _association: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for a single volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume to retrieve.
    pub fn get(&self, volume_id: &str) -> VolumeGetCall<'a, C, NC, A> {
        VolumeGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _user_library_consistent_read: Default::default(),
            _source: Default::default(),
            _projection: Default::default(),
            _partner: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of recommended books for the current user.
    pub fn recommended_list(&self) -> VolumeRecommendedListCall<'a, C, NC, A> {
        VolumeRecommendedListCall {
            hub: self.hub,
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dictionary* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_offline_metadata(...)`
/// // to build up your call.
/// let rb = hub.dictionary();
/// # }
/// ```
pub struct DictionaryMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for DictionaryMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> DictionaryMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of offline dictionary meatadata available
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to request the data.
    pub fn list_offline_metadata(&self, cpksver: &str) -> DictionaryListOfflineMetadataCall<'a, C, NC, A> {
        DictionaryListOfflineMetadataCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *bookshelve* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `volumes_list(...)`
/// // to build up your call.
/// let rb = hub.bookshelves();
/// # }
/// ```
pub struct BookshelveMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for BookshelveMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> BookshelveMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves volumes in a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelf volumes.
    /// * `shelf` - ID of bookshelf to retrieve volumes.
    pub fn volumes_list(&self, user_id: &str, shelf: &str) -> BookshelveVolumeListCall<'a, C, NC, A> {
        BookshelveVolumeListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of public bookshelves for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    pub fn list(&self, user_id: &str) -> BookshelveListCall<'a, C, NC, A> {
        BookshelveListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn get(&self, user_id: &str, shelf: &str) -> BookshelveGetCall<'a, C, NC, A> {
        BookshelveGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *promooffer* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accept(...)`, `dismiss(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.promooffer();
/// # }
/// ```
pub struct PromoofferMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for PromoofferMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> PromoofferMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    pub fn accept(&self) -> PromoofferAcceptCall<'a, C, NC, A> {
        PromoofferAcceptCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    pub fn dismiss(&self) -> PromoofferDismisCall<'a, C, NC, A> {
        PromoofferDismisCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of promo offers available to the user
    pub fn get(&self) -> PromoofferGetCall<'a, C, NC, A> {
        PromoofferGetCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *onboarding* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_categories(...)` and `list_category_volumes(...)`
/// // to build up your call.
/// let rb = hub.onboarding();
/// # }
/// ```
pub struct OnboardingMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for OnboardingMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> OnboardingMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List categories for onboarding experience.
    pub fn list_categories(&self) -> OnboardingListCategoryCall<'a, C, NC, A> {
        OnboardingListCategoryCall {
            hub: self.hub,
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List available volumes under categories for onboarding experience.
    pub fn list_category_volumes(&self) -> OnboardingListCategoryVolumeCall<'a, C, NC, A> {
        OnboardingListCategoryVolumeCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _locale: Default::default(),
            _category_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *myconfig* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_user_settings(...)`, `release_download_access(...)`, `request_access(...)`, `sync_volume_licenses(...)` and `update_user_settings(...)`
/// // to build up your call.
/// let rb = hub.myconfig();
/// # }
/// ```
pub struct MyconfigMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for MyconfigMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> MyconfigMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request concurrent and download access restrictions.
    /// 
    /// # Arguments
    ///
    /// * `source` - String to identify the originator of this request.
    /// * `volumeId` - The volume to request concurrent/download restrictions for.
    /// * `nonce` - The client nonce value.
    /// * `cpksver` - The device/version ID from which to request the restrictions.
    pub fn request_access(&self, source: &str, volume_id: &str, nonce: &str, cpksver: &str) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        MyconfigRequestAccesCall {
            hub: self.hub,
            _source: source.to_string(),
            _volume_id: volume_id.to_string(),
            _nonce: nonce.to_string(),
            _cpksver: cpksver.to_string(),
            _locale: Default::default(),
            _license_types: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Release downloaded content access restriction.
    /// 
    /// # Arguments
    ///
    /// * `volumeIds` - The volume(s) to release restrictions for.
    /// * `cpksver` - The device/version ID from which to release the restriction.
    pub fn release_download_access(&self, volume_ids: &Vec<String>, cpksver: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> {
        MyconfigReleaseDownloadAccesCall {
            hub: self.hub,
            _volume_ids: volume_ids.clone(),
            _cpksver: cpksver.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request downloaded content access for specified volumes on the My eBooks shelf.
    /// 
    /// # Arguments
    ///
    /// * `source` - String to identify the originator of this request.
    /// * `nonce` - The client nonce value.
    /// * `cpksver` - The device/version ID from which to release the restriction.
    pub fn sync_volume_licenses(&self, source: &str, nonce: &str, cpksver: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        MyconfigSyncVolumeLicenseCall {
            hub: self.hub,
            _source: source.to_string(),
            _nonce: nonce.to_string(),
            _cpksver: cpksver.to_string(),
            _volume_ids: Default::default(),
            _show_preorders: Default::default(),
            _locale: Default::default(),
            _features: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current settings for the user.
    pub fn get_user_settings(&self) -> MyconfigGetUserSettingCall<'a, C, NC, A> {
        MyconfigGetUserSettingCall {
            hub: self.hub,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the settings for the user. Unspecified sub-objects will retain the existing value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_user_settings(&self, request: &Usersettings) -> MyconfigUpdateUserSettingCall<'a, C, NC, A> {
        MyconfigUpdateUserSettingCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *mylibrary* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotations_delete(...)`, `annotations_insert(...)`, `annotations_list(...)`, `annotations_summary(...)`, `annotations_update(...)`, `bookshelves_add_volume(...)`, `bookshelves_clear_volumes(...)`, `bookshelves_get(...)`, `bookshelves_list(...)`, `bookshelves_move_volume(...)`, `bookshelves_remove_volume(...)`, `bookshelves_volumes_list(...)`, `readingpositions_get(...)` and `readingpositions_set_position(...)`
/// // to build up your call.
/// let rb = hub.mylibrary();
/// # }
/// ```
pub struct MylibraryMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for MylibraryMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears all volumes from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    pub fn bookshelves_clear_volumes(&self, shelf: &str) -> MylibraryBookshelveClearVolumeCall<'a, C, NC, A> {
        MylibraryBookshelveClearVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a volume within a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf with the volume.
    /// * `volumeId` - ID of volume to move.
    /// * `volumePosition` - Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)
    pub fn bookshelves_move_volume(&self, shelf: &str, volume_id: &str, volume_position: i32) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> {
        MylibraryBookshelveMoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _volume_position: volume_position,
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for volumes on a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - The bookshelf ID or name retrieve volumes for.
    pub fn bookshelves_volumes_list(&self, shelf: &str) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        MylibraryBookshelveVolumeListCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _q: Default::default(),
            _projection: Default::default(),
            _max_results: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the summary of specified layers.
    /// 
    /// # Arguments
    ///
    /// * `layerIds` - Array of layer IDs to get the summary for.
    /// * `volumeId` - Volume id to get the summary for.
    pub fn annotations_summary(&self, layer_ids: &Vec<String>, volume_id: &str) -> MylibraryAnnotationSummaryCall<'a, C, NC, A> {
        MylibraryAnnotationSummaryCall {
            hub: self.hub,
            _layer_ids: layer_ids.clone(),
            _volume_id: volume_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an annotation.
    /// 
    /// # Arguments
    ///
    /// * `annotationId` - The ID for the annotation to delete.
    pub fn annotations_delete(&self, annotation_id: &str) -> MylibraryAnnotationDeleteCall<'a, C, NC, A> {
        MylibraryAnnotationDeleteCall {
            hub: self.hub,
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a volume to a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to which to add a volume.
    /// * `volumeId` - ID of volume to add.
    pub fn bookshelves_add_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> {
        MylibraryBookshelveAddVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotations_insert(&self, request: &Annotation) -> MylibraryAnnotationInsertCall<'a, C, NC, A> {
        MylibraryAnnotationInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _source: Default::default(),
            _show_only_summary_in_response: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a volume from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    /// * `volumeId` - ID of volume to remove.
    pub fn bookshelves_remove_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> {
        MylibraryBookshelveRemoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of annotations, possibly filtered.
    pub fn annotations_list(&self) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        MylibraryAnnotationListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _layer_ids: Default::default(),
            _layer_id: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `annotationId` - The ID for the annotation to update.
    pub fn annotations_update(&self, request: &Annotation, annotation_id: &str) -> MylibraryAnnotationUpdateCall<'a, C, NC, A> {
        MylibraryAnnotationUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to update the reading position.
    /// * `timestamp` - RFC 3339 UTC format timestamp associated with this reading position.
    /// * `position` - Position string for the new volume reading position.
    pub fn readingpositions_set_position(&self, volume_id: &str, timestamp: &str, position: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        MylibraryReadingpositionSetPositionCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _timestamp: timestamp.to_string(),
            _position: position.to_string(),
            _source: Default::default(),
            _device_cookie: Default::default(),
            _content_version: Default::default(),
            _action: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf belonging to the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn bookshelves_get(&self, shelf: &str) -> MylibraryBookshelveGetCall<'a, C, NC, A> {
        MylibraryBookshelveGetCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of bookshelves belonging to the authenticated user.
    pub fn bookshelves_list(&self) -> MylibraryBookshelveListCall<'a, C, NC, A> {
        MylibraryBookshelveListCall {
            hub: self.hub,
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to retrieve a reading position.
    pub fn readingpositions_get(&self, volume_id: &str) -> MylibraryReadingpositionGetCall<'a, C, NC, A> {
        MylibraryReadingpositionGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *cloudloading* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_book(...)`, `delete_book(...)` and `update_book(...)`
/// // to build up your call.
/// let rb = hub.cloudloading();
/// # }
/// ```
pub struct CloudloadingMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for CloudloadingMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> CloudloadingMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    pub fn add_book(&self) -> CloudloadingAddBookCall<'a, C, NC, A> {
        CloudloadingAddBookCall {
            hub: self.hub,
            _upload_client_token: Default::default(),
            _name: Default::default(),
            _mime_type: Default::default(),
            _drive_document_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_book(&self, request: &BooksCloudloadingResource) -> CloudloadingUpdateBookCall<'a, C, NC, A> {
        CloudloadingUpdateBookCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove the book and its contents
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The id of the book to be removed.
    pub fn delete_book(&self, volume_id: &str) -> CloudloadingDeleteBookCall<'a, C, NC, A> {
        CloudloadingDeleteBookCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets the annotation data.
///
/// A builder for the *annotationData.get* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().annotation_data_get("volumeId", "layerId", "annotationDataId", "contentVersion")
///              .w(-33)
///              .source("invidunt")
///              .scale(-82)
///              .locale("accusam")
///              .h(-56)
///              .allow_web_definitions(true)
///              .doit();
/// # }
/// ```
pub struct LayerAnnotationDataGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _layer_id: String,
    _annotation_data_id: String,
    _content_version: String,
    _w: Option<i32>,
    _source: Option<String>,
    _scale: Option<i32>,
    _locale: Option<String>,
    _h: Option<i32>,
    _allow_web_definitions: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LayerAnnotationDataGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LayerAnnotationDataGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotationdata)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.annotationData.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("annotationDataId", self._annotation_data_id.to_string()));
        params.push(("contentVersion", self._content_version.to_string()));
        if let Some(value) = self._w {
            params.push(("w", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._scale {
            params.push(("scale", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._h {
            params.push(("h", value.to_string()));
        }
        if let Some(value) = self._allow_web_definitions {
            params.push(("allowWebDefinitions", value.to_string()));
        }
        for &field in ["alt", "volumeId", "layerId", "annotationDataId", "contentVersion", "w", "source", "scale", "locale", "h", "allowWebDefinitions"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}/layers/{layerId}/data/{annotationDataId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId"), ("{annotationDataId}", "annotationDataId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["volumeId", "layerId", "annotationDataId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume to retrieve annotations for.
    pub fn volume_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID for the layer to get the annotations.
    pub fn layer_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// Sets the *annotation data id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the annotation data to retrieve.
    pub fn annotation_data_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._annotation_data_id = new_value.to_string();
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The content version for the volume you are trying to retrieve.
    pub fn content_version(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._content_version = new_value.to_string();
        self
    }
    /// Sets the *w* query property to the given value.
    ///
    /// 
    /// The requested pixel width for any images. If width is provided height must also be provided.
    pub fn w(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._w = Some(new_value);
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *scale* query property to the given value.
    ///
    /// 
    /// The requested scale for the image.
    pub fn scale(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._scale = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    pub fn locale(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *h* query property to the given value.
    ///
    /// 
    /// The requested pixel height for any images. If height is provided width must also be provided.
    pub fn h(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._h = Some(new_value);
        self
    }
    /// Sets the *allow web definitions* query property to the given value.
    ///
    /// 
    /// For the dictionary layer. Whether or not to allow web definitions.
    pub fn allow_web_definitions(mut self, new_value: bool) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._allow_web_definitions = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerAnnotationDataGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerAnnotationDataGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LayerAnnotationDataGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the volume annotation.
///
/// A builder for the *volumeAnnotations.get* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().volume_annotations_get("volumeId", "layerId", "annotationId")
///              .source("eirmod")
///              .locale("sanctus")
///              .doit();
/// # }
/// ```
pub struct LayerVolumeAnnotationGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _layer_id: String,
    _annotation_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LayerVolumeAnnotationGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LayerVolumeAnnotationGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumeannotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.volumeAnnotations.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("annotationId", self._annotation_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "volumeId", "layerId", "annotationId", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}/layers/{layerId}/annotations/{annotationId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId"), ("{annotationId}", "annotationId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["volumeId", "layerId", "annotationId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume to retrieve annotations for.
    pub fn volume_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID for the layer to get the annotations.
    pub fn layer_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the volume annotation to retrieve.
    pub fn annotation_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    pub fn locale(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerVolumeAnnotationGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LayerVolumeAnnotationGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List the layer summaries for a volume.
///
/// A builder for the *list* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().list("volumeId")
///              .source("amet")
///              .page_token("et")
///              .max_results(56)
///              .content_version("ut")
///              .doit();
/// # }
/// ```
pub struct LayerListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _source: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LayerListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LayerListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Layersummaries)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "source", "pageToken", "maxResults", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}/layersummary".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["volumeId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume to retrieve layers for.
    pub fn volume_id(mut self, new_value: &str) -> LayerListCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> LayerListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The value of the nextToken from the previous page.
    pub fn page_token(mut self, new_value: &str) -> LayerListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return
    pub fn max_results(mut self, new_value: u32) -> LayerListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// 
    /// The content version for the requested volume.
    pub fn content_version(mut self, new_value: &str) -> LayerListCall<'a, C, NC, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LayerListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the layer summary for a volume.
///
/// A builder for the *get* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().get("volumeId", "summaryId")
///              .source("dolor")
///              .content_version("dolor")
///              .doit();
/// # }
/// ```
pub struct LayerGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _summary_id: String,
    _source: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LayerGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LayerGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Layersummary)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("summaryId", self._summary_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "summaryId", "source", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}/layersummary/{summaryId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{summaryId}", "summaryId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["volumeId", "summaryId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume to retrieve layers for.
    pub fn volume_id(mut self, new_value: &str) -> LayerGetCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *summary id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID for the layer to get the summary for.
    pub fn summary_id(mut self, new_value: &str) -> LayerGetCall<'a, C, NC, A> {
        self._summary_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> LayerGetCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// 
    /// The content version for the requested volume.
    pub fn content_version(mut self, new_value: &str) -> LayerGetCall<'a, C, NC, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LayerGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the volume annotations for a volume and layer.
///
/// A builder for the *volumeAnnotations.list* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().volume_annotations_list("volumeId", "layerId", "contentVersion")
///              .volume_annotations_version("amet.")
///              .updated_min("voluptua.")
///              .updated_max("Lorem")
///              .start_position("gubergren")
///              .start_offset("justo")
///              .source("sit")
///              .show_deleted(true)
///              .page_token("diam")
///              .max_results(35)
///              .locale("consetetur")
///              .end_position("sadipscing")
///              .end_offset("vero")
///              .doit();
/// # }
/// ```
pub struct LayerVolumeAnnotationListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _layer_id: String,
    _content_version: String,
    _volume_annotations_version: Option<String>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _start_position: Option<String>,
    _start_offset: Option<String>,
    _source: Option<String>,
    _show_deleted: Option<bool>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _end_position: Option<String>,
    _end_offset: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LayerVolumeAnnotationListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LayerVolumeAnnotationListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumeannotations)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.volumeAnnotations.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((17 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("contentVersion", self._content_version.to_string()));
        if let Some(value) = self._volume_annotations_version {
            params.push(("volumeAnnotationsVersion", value.to_string()));
        }
        if let Some(value) = self._updated_min {
            params.push(("updatedMin", value.to_string()));
        }
        if let Some(value) = self._updated_max {
            params.push(("updatedMax", value.to_string()));
        }
        if let Some(value) = self._start_position {
            params.push(("startPosition", value.to_string()));
        }
        if let Some(value) = self._start_offset {
            params.push(("startOffset", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_deleted {
            params.push(("showDeleted", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._end_position {
            params.push(("endPosition", value.to_string()));
        }
        if let Some(value) = self._end_offset {
            params.push(("endOffset", value.to_string()));
        }
        for &field in ["alt", "volumeId", "layerId", "contentVersion", "volumeAnnotationsVersion", "updatedMin", "updatedMax", "startPosition", "startOffset", "source", "showDeleted", "pageToken", "maxResults", "locale", "endPosition", "endOffset"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}/layers/{layerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["volumeId", "layerId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume to retrieve annotations for.
    pub fn volume_id(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID for the layer to get the annotations.
    pub fn layer_id(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The content version for the requested volume.
    pub fn content_version(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._content_version = new_value.to_string();
        self
    }
    /// Sets the *volume annotations version* query property to the given value.
    ///
    /// 
    /// The version of the volume annotations that you are requesting.
    pub fn volume_annotations_version(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._volume_annotations_version = Some(new_value.to_string());
        self
    }
    /// Sets the *updated min* query property to the given value.
    ///
    /// 
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    pub fn updated_min(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// Sets the *updated max* query property to the given value.
    ///
    /// 
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    pub fn updated_max(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// Sets the *start position* query property to the given value.
    ///
    /// 
    /// The start position to start retrieving data from.
    pub fn start_position(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._start_position = Some(new_value.to_string());
        self
    }
    /// Sets the *start offset* query property to the given value.
    ///
    /// 
    /// The start offset to start retrieving data from.
    pub fn start_offset(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._start_offset = Some(new_value.to_string());
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *show deleted* query property to the given value.
    ///
    /// 
    /// Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false.
    pub fn show_deleted(mut self, new_value: bool) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._show_deleted = Some(new_value);
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The value of the nextToken from the previous page.
    pub fn page_token(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return
    pub fn max_results(mut self, new_value: u32) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    pub fn locale(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *end position* query property to the given value.
    ///
    /// 
    /// The end position to end retrieving data from.
    pub fn end_position(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._end_position = Some(new_value.to_string());
        self
    }
    /// Sets the *end offset* query property to the given value.
    ///
    /// 
    /// The end offset to end retrieving data from.
    pub fn end_offset(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._end_offset = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerVolumeAnnotationListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerVolumeAnnotationListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LayerVolumeAnnotationListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the annotation data for a volume and layer.
///
/// A builder for the *annotationData.list* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().annotation_data_list("volumeId", "layerId", "contentVersion")
///              .w(-84)
///              .updated_min("duo")
///              .updated_max("aliquyam")
///              .source("Lorem")
///              .scale(-17)
///              .page_token("clita")
///              .max_results(56)
///              .locale("takimata")
///              .h(-40)
///              .add_annotation_data_id("kasd")
///              .doit();
/// # }
/// ```
pub struct LayerAnnotationDataListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _layer_id: String,
    _content_version: String,
    _w: Option<i32>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _source: Option<String>,
    _scale: Option<i32>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _h: Option<i32>,
    _annotation_data_id: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LayerAnnotationDataListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LayerAnnotationDataListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotationsdata)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.annotationData.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((15 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("contentVersion", self._content_version.to_string()));
        if let Some(value) = self._w {
            params.push(("w", value.to_string()));
        }
        if let Some(value) = self._updated_min {
            params.push(("updatedMin", value.to_string()));
        }
        if let Some(value) = self._updated_max {
            params.push(("updatedMax", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._scale {
            params.push(("scale", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._h {
            params.push(("h", value.to_string()));
        }
        if self._annotation_data_id.len() > 0 {
            let mut s = String::new();
            for f in self._annotation_data_id.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("annotationDataId", s));
        }
        for &field in ["alt", "volumeId", "layerId", "contentVersion", "w", "updatedMin", "updatedMax", "source", "scale", "pageToken", "maxResults", "locale", "h", "annotationDataId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}/layers/{layerId}/data".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["volumeId", "layerId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume to retrieve annotation data for.
    pub fn volume_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID for the layer to get the annotation data.
    pub fn layer_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The content version for the requested volume.
    pub fn content_version(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._content_version = new_value.to_string();
        self
    }
    /// Sets the *w* query property to the given value.
    ///
    /// 
    /// The requested pixel width for any images. If width is provided height must also be provided.
    pub fn w(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._w = Some(new_value);
        self
    }
    /// Sets the *updated min* query property to the given value.
    ///
    /// 
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    pub fn updated_min(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// Sets the *updated max* query property to the given value.
    ///
    /// 
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    pub fn updated_max(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *scale* query property to the given value.
    ///
    /// 
    /// The requested scale for the image.
    pub fn scale(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._scale = Some(new_value);
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The value of the nextToken from the previous page.
    pub fn page_token(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return
    pub fn max_results(mut self, new_value: u32) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    pub fn locale(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *h* query property to the given value.
    ///
    /// 
    /// The requested pixel height for any images. If height is provided width must also be provided.
    pub fn h(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._h = Some(new_value);
        self
    }
    /// Append the given value to the *annotation data id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// The list of Annotation Data Ids to retrieve. Pagination is ignored if this is set.
    pub fn add_annotation_data_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._annotation_data_id.push(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerAnnotationDataListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerAnnotationDataListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LayerAnnotationDataListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Rate a recommended book for the current user.
///
/// A builder for the *recommended.rate* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().recommended_rate("rating", "volumeId")
///              .source("At")
///              .locale("labore")
///              .doit();
/// # }
/// ```
pub struct VolumeRecommendedRateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _rating: String,
    _volume_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VolumeRecommendedRateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeRecommendedRateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BooksVolumesRecommendedRateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.recommended.rate", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("rating", self._rating.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "rating", "volumeId", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/recommended/rate".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *rating* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Rating to be given to the volume.
    pub fn rating(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, NC, A> {
        self._rating = new_value.to_string();
        self
    }
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of the source volume.
    pub fn volume_id(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    pub fn locale(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeRecommendedRateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeRecommendedRateCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VolumeRecommendedRateCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Return a list of books in My Library.
///
/// A builder for the *mybooks.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().mybooks_list()
///              .start_index(64)
///              .source("ea")
///              .add_processing_state("sadipscing")
///              .max_results(35)
///              .locale("dolore")
///              .add_acquire_method("nonumy")
///              .doit();
/// # }
/// ```
pub struct VolumeMybookListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _start_index: Option<u32>,
    _source: Option<String>,
    _processing_state: Vec<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _acquire_method: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VolumeMybookListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeMybookListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.mybooks.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if self._processing_state.len() > 0 {
            let mut s = String::new();
            for f in self._processing_state.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("processingState", s));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if self._acquire_method.len() > 0 {
            let mut s = String::new();
            for f in self._acquire_method.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("acquireMethod", s));
        }
        for &field in ["alt", "startIndex", "source", "processingState", "maxResults", "locale", "acquireMethod"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/mybooks".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *start index* query property to the given value.
    ///
    /// 
    /// Index of the first result to return (starts at 0)
    pub fn start_index(mut self, new_value: u32) -> VolumeMybookListCall<'a, C, NC, A> {
        self._start_index = Some(new_value);
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *processing state* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// The processing state of the user uploaded volumes to be returned. Applicable only if the UPLOADED is specified in the acquireMethod.
    pub fn add_processing_state(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, NC, A> {
        self._processing_state.push(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return.
    pub fn max_results(mut self, new_value: u32) -> VolumeMybookListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1 language and ISO-3166-1 country code. Ex:'en_US'. Used for generating recommendations.
    pub fn locale(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *acquire method* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// How the book was aquired
    pub fn add_acquire_method(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, NC, A> {
        self._acquire_method.push(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeMybookListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeMybookListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VolumeMybookListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Performs a book search.
///
/// A builder for the *list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().list("q")
///              .start_index(19)
///              .source("sit")
///              .show_preorders(true)
///              .projection("consetetur")
///              .print_type("labore")
///              .partner("sed")
///              .order_by("ea")
///              .max_results(39)
///              .library_restrict("aliquyam")
///              .lang_restrict("eos")
///              .filter("tempor")
///              .download("sea")
///              .doit();
/// # }
/// ```
pub struct VolumeListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _q: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _projection: Option<String>,
    _print_type: Option<String>,
    _partner: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _library_restrict: Option<String>,
    _lang_restrict: Option<String>,
    _filter: Option<String>,
    _download: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VolumeListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((15 + self._additional_params.len()));
        params.push(("q", self._q.to_string()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._print_type {
            params.push(("printType", value.to_string()));
        }
        if let Some(value) = self._partner {
            params.push(("partner", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._library_restrict {
            params.push(("libraryRestrict", value.to_string()));
        }
        if let Some(value) = self._lang_restrict {
            params.push(("langRestrict", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        if let Some(value) = self._download {
            params.push(("download", value.to_string()));
        }
        for &field in ["alt", "q", "startIndex", "source", "showPreorders", "projection", "printType", "partner", "orderBy", "maxResults", "libraryRestrict", "langRestrict", "filter", "download"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *q* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Full-text search query string.
    pub fn q(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._q = new_value.to_string();
        self
    }
    /// Sets the *start index* query property to the given value.
    ///
    /// 
    /// Index of the first result to return (starts at 0)
    pub fn start_index(mut self, new_value: u32) -> VolumeListCall<'a, C, NC, A> {
        self._start_index = Some(new_value);
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *show preorders* query property to the given value.
    ///
    /// 
    /// Set to true to show books available for preorder. Defaults to false.
    pub fn show_preorders(mut self, new_value: bool) -> VolumeListCall<'a, C, NC, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Restrict information returned to a set of selected fields.
    pub fn projection(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *print type* query property to the given value.
    ///
    /// 
    /// Restrict to books or magazines.
    pub fn print_type(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._print_type = Some(new_value.to_string());
        self
    }
    /// Sets the *partner* query property to the given value.
    ///
    /// 
    /// Restrict and brand results for partner ID.
    pub fn partner(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._partner = Some(new_value.to_string());
        self
    }
    /// Sets the *order by* query property to the given value.
    ///
    /// 
    /// Sort search results.
    pub fn order_by(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return.
    pub fn max_results(mut self, new_value: u32) -> VolumeListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *library restrict* query property to the given value.
    ///
    /// 
    /// Restrict search to this user's library.
    pub fn library_restrict(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._library_restrict = Some(new_value.to_string());
        self
    }
    /// Sets the *lang restrict* query property to the given value.
    ///
    /// 
    /// Restrict results to books with this language code.
    pub fn lang_restrict(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._lang_restrict = Some(new_value.to_string());
        self
    }
    /// Sets the *filter* query property to the given value.
    ///
    /// 
    /// Filter search results.
    pub fn filter(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Sets the *download* query property to the given value.
    ///
    /// 
    /// Restrict to volumes by download availability.
    pub fn download(mut self, new_value: &str) -> VolumeListCall<'a, C, NC, A> {
        self._download = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VolumeListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Return a list of books uploaded by the current user.
///
/// A builder for the *useruploaded.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().useruploaded_list()
///              .add_volume_id("labore")
///              .start_index(47)
///              .source("aliquyam")
///              .add_processing_state("dolores")
///              .max_results(3)
///              .locale("diam")
///              .doit();
/// # }
/// ```
pub struct VolumeUseruploadedListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: Vec<String>,
    _start_index: Option<u32>,
    _source: Option<String>,
    _processing_state: Vec<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VolumeUseruploadedListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeUseruploadedListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.useruploaded.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        if self._volume_id.len() > 0 {
            let mut s = String::new();
            for f in self._volume_id.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("volumeId", s));
        }
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if self._processing_state.len() > 0 {
            let mut s = String::new();
            for f in self._processing_state.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("processingState", s));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "volumeId", "startIndex", "source", "processingState", "maxResults", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/useruploaded".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Append the given value to the *volume id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// The ids of the volumes to be returned. If not specified all that match the processingState are returned.
    pub fn add_volume_id(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        self._volume_id.push(new_value.to_string());
        self
    }
    /// Sets the *start index* query property to the given value.
    ///
    /// 
    /// Index of the first result to return (starts at 0)
    pub fn start_index(mut self, new_value: u32) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        self._start_index = Some(new_value);
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *processing state* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// The processing state of the user uploaded volumes to be returned.
    pub fn add_processing_state(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        self._processing_state.push(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return.
    pub fn max_results(mut self, new_value: u32) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    pub fn locale(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeUseruploadedListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeUseruploadedListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VolumeUseruploadedListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Return a list of associated books.
///
/// A builder for the *associated.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().associated_list("volumeId")
///              .source("justo")
///              .locale("est")
///              .association("amet")
///              .doit();
/// # }
/// ```
pub struct VolumeAssociatedListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _association: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VolumeAssociatedListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeAssociatedListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.associated.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._association {
            params.push(("association", value.to_string()));
        }
        for &field in ["alt", "volumeId", "source", "locale", "association"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}/associated".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["volumeId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of the source volume.
    pub fn volume_id(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    pub fn locale(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *association* query property to the given value.
    ///
    /// 
    /// Association type.
    pub fn association(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, NC, A> {
        self._association = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeAssociatedListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeAssociatedListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VolumeAssociatedListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets volume information for a single volume.
///
/// A builder for the *get* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().get("volumeId")
///              .user_library_consistent_read(true)
///              .source("diam")
///              .projection("justo")
///              .partner("est")
///              .country("clita")
///              .doit();
/// # }
/// ```
pub struct VolumeGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _user_library_consistent_read: Option<bool>,
    _source: Option<String>,
    _projection: Option<String>,
    _partner: Option<String>,
    _country: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VolumeGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volume)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._user_library_consistent_read {
            params.push(("user_library_consistent_read", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._partner {
            params.push(("partner", value.to_string()));
        }
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        for &field in ["alt", "volumeId", "user_library_consistent_read", "source", "projection", "partner", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/{volumeId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["volumeId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of volume to retrieve.
    pub fn volume_id(mut self, new_value: &str) -> VolumeGetCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *user_library_consistent_read* query property to the given value.
    ///
    /// 
    pub fn user_library_consistent_read(mut self, new_value: bool) -> VolumeGetCall<'a, C, NC, A> {
        self._user_library_consistent_read = Some(new_value);
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> VolumeGetCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Restrict information returned to a set of selected fields.
    pub fn projection(mut self, new_value: &str) -> VolumeGetCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *partner* query property to the given value.
    ///
    /// 
    /// Brand results for partner ID.
    pub fn partner(mut self, new_value: &str) -> VolumeGetCall<'a, C, NC, A> {
        self._partner = Some(new_value.to_string());
        self
    }
    /// Sets the *country* query property to the given value.
    ///
    /// 
    /// ISO-3166-1 code to override the IP-based location.
    pub fn country(mut self, new_value: &str) -> VolumeGetCall<'a, C, NC, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VolumeGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Return a list of recommended books for the current user.
///
/// A builder for the *recommended.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().recommended_list()
///              .source("invidunt")
///              .locale("ut")
///              .doit();
/// # }
/// ```
pub struct VolumeRecommendedListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VolumeRecommendedListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VolumeRecommendedListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.recommended.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/volumes/recommended".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    pub fn locale(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeRecommendedListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeRecommendedListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VolumeRecommendedListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Returns a list of offline dictionary meatadata available
///
/// A builder for the *listOfflineMetadata* method supported by a *dictionary* resource.
/// It is not used directly, but through a `DictionaryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.dictionary().list_offline_metadata("cpksver")
///              .doit();
/// # }
/// ```
pub struct DictionaryListOfflineMetadataCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _cpksver: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DictionaryListOfflineMetadataCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DictionaryListOfflineMetadataCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Metadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.dictionary.listOfflineMetadata", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("cpksver", self._cpksver.to_string()));
        for &field in ["alt", "cpksver"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/dictionary/listOfflineMetadata".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The device/version ID from which to request the data.
    pub fn cpksver(mut self, new_value: &str) -> DictionaryListOfflineMetadataCall<'a, C, NC, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DictionaryListOfflineMetadataCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DictionaryListOfflineMetadataCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DictionaryListOfflineMetadataCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves volumes in a specific bookshelf for the specified user.
///
/// A builder for the *volumes.list* method supported by a *bookshelve* resource.
/// It is not used directly, but through a `BookshelveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().volumes_list("userId", "shelf")
///              .start_index(82)
///              .source("sed")
///              .show_preorders(true)
///              .max_results(34)
///              .doit();
/// # }
/// ```
pub struct BookshelveVolumeListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _user_id: String,
    _shelf: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BookshelveVolumeListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BookshelveVolumeListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.bookshelves.volumes.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "userId", "shelf", "startIndex", "source", "showPreorders", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/users/{userId}/bookshelves/{shelf}/volumes".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of user for whom to retrieve bookshelf volumes.
    pub fn user_id(mut self, new_value: &str) -> BookshelveVolumeListCall<'a, C, NC, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of bookshelf to retrieve volumes.
    pub fn shelf(mut self, new_value: &str) -> BookshelveVolumeListCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *start index* query property to the given value.
    ///
    /// 
    /// Index of the first element to return (starts at 0)
    pub fn start_index(mut self, new_value: u32) -> BookshelveVolumeListCall<'a, C, NC, A> {
        self._start_index = Some(new_value);
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> BookshelveVolumeListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *show preorders* query property to the given value.
    ///
    /// 
    /// Set to true to show pre-ordered books. Defaults to false.
    pub fn show_preorders(mut self, new_value: bool) -> BookshelveVolumeListCall<'a, C, NC, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return
    pub fn max_results(mut self, new_value: u32) -> BookshelveVolumeListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BookshelveVolumeListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> BookshelveVolumeListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> BookshelveVolumeListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves a list of public bookshelves for the specified user.
///
/// A builder for the *list* method supported by a *bookshelve* resource.
/// It is not used directly, but through a `BookshelveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().list("userId")
///              .source("et")
///              .doit();
/// # }
/// ```
pub struct BookshelveListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _user_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BookshelveListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BookshelveListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelves)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.bookshelves.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "userId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/users/{userId}/bookshelves".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of user for whom to retrieve bookshelves.
    pub fn user_id(mut self, new_value: &str) -> BookshelveListCall<'a, C, NC, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> BookshelveListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BookshelveListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> BookshelveListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> BookshelveListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves metadata for a specific bookshelf for the specified user.
///
/// A builder for the *get* method supported by a *bookshelve* resource.
/// It is not used directly, but through a `BookshelveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().get("userId", "shelf")
///              .source("kasd")
///              .doit();
/// # }
/// ```
pub struct BookshelveGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _user_id: String,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BookshelveGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BookshelveGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelf)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.bookshelves.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "userId", "shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/users/{userId}/bookshelves/{shelf}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of user for whom to retrieve bookshelves.
    pub fn user_id(mut self, new_value: &str) -> BookshelveGetCall<'a, C, NC, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of bookshelf to retrieve.
    pub fn shelf(mut self, new_value: &str) -> BookshelveGetCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> BookshelveGetCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BookshelveGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> BookshelveGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> BookshelveGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *accept* method supported by a *promooffer* resource.
/// It is not used directly, but through a `PromoofferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().accept()
///              .volume_id("invidunt")
///              .serial("rebum.")
///              .product("Lorem")
///              .offer_id("clita")
///              .model("invidunt")
///              .manufacturer("eirmod")
///              .device("At")
///              .android_id("consetetur")
///              .doit();
/// # }
/// ```
pub struct PromoofferAcceptCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: Option<String>,
    _serial: Option<String>,
    _product: Option<String>,
    _offer_id: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PromoofferAcceptCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PromoofferAcceptCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.promooffer.accept", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        if let Some(value) = self._volume_id {
            params.push(("volumeId", value.to_string()));
        }
        if let Some(value) = self._serial {
            params.push(("serial", value.to_string()));
        }
        if let Some(value) = self._product {
            params.push(("product", value.to_string()));
        }
        if let Some(value) = self._offer_id {
            params.push(("offerId", value.to_string()));
        }
        if let Some(value) = self._model {
            params.push(("model", value.to_string()));
        }
        if let Some(value) = self._manufacturer {
            params.push(("manufacturer", value.to_string()));
        }
        if let Some(value) = self._device {
            params.push(("device", value.to_string()));
        }
        if let Some(value) = self._android_id {
            params.push(("androidId", value.to_string()));
        }
        for &field in ["volumeId", "serial", "product", "offerId", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/promooffer/accept".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* query property to the given value.
    ///
    /// 
    /// Volume id to exercise the offer
    pub fn volume_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// Sets the *serial* query property to the given value.
    ///
    /// 
    /// device serial
    pub fn serial(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// Sets the *product* query property to the given value.
    ///
    /// 
    /// device product
    pub fn product(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._product = Some(new_value.to_string());
        self
    }
    /// Sets the *offer id* query property to the given value.
    ///
    /// 
    pub fn offer_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._offer_id = Some(new_value.to_string());
        self
    }
    /// Sets the *model* query property to the given value.
    ///
    /// 
    /// device model
    pub fn model(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._model = Some(new_value.to_string());
        self
    }
    /// Sets the *manufacturer* query property to the given value.
    ///
    /// 
    /// device manufacturer
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// Sets the *device* query property to the given value.
    ///
    /// 
    /// device device
    pub fn device(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._device = Some(new_value.to_string());
        self
    }
    /// Sets the *android id* query property to the given value.
    ///
    /// 
    /// device android_id
    pub fn android_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PromoofferAcceptCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferAcceptCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PromoofferAcceptCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *dismiss* method supported by a *promooffer* resource.
/// It is not used directly, but through a `PromoofferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().dismiss()
///              .serial("et")
///              .product("sed")
///              .offer_id("sit")
///              .model("takimata")
///              .manufacturer("elitr")
///              .device("nonumy")
///              .android_id("rebum.")
///              .doit();
/// # }
/// ```
pub struct PromoofferDismisCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _serial: Option<String>,
    _product: Option<String>,
    _offer_id: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PromoofferDismisCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PromoofferDismisCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.promooffer.dismiss", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        if let Some(value) = self._serial {
            params.push(("serial", value.to_string()));
        }
        if let Some(value) = self._product {
            params.push(("product", value.to_string()));
        }
        if let Some(value) = self._offer_id {
            params.push(("offerId", value.to_string()));
        }
        if let Some(value) = self._model {
            params.push(("model", value.to_string()));
        }
        if let Some(value) = self._manufacturer {
            params.push(("manufacturer", value.to_string()));
        }
        if let Some(value) = self._device {
            params.push(("device", value.to_string()));
        }
        if let Some(value) = self._android_id {
            params.push(("androidId", value.to_string()));
        }
        for &field in ["serial", "product", "offerId", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/promooffer/dismiss".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *serial* query property to the given value.
    ///
    /// 
    /// device serial
    pub fn serial(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, NC, A> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// Sets the *product* query property to the given value.
    ///
    /// 
    /// device product
    pub fn product(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, NC, A> {
        self._product = Some(new_value.to_string());
        self
    }
    /// Sets the *offer id* query property to the given value.
    ///
    /// 
    /// Offer to dimiss
    pub fn offer_id(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, NC, A> {
        self._offer_id = Some(new_value.to_string());
        self
    }
    /// Sets the *model* query property to the given value.
    ///
    /// 
    /// device model
    pub fn model(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, NC, A> {
        self._model = Some(new_value.to_string());
        self
    }
    /// Sets the *manufacturer* query property to the given value.
    ///
    /// 
    /// device manufacturer
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, NC, A> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// Sets the *device* query property to the given value.
    ///
    /// 
    /// device device
    pub fn device(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, NC, A> {
        self._device = Some(new_value.to_string());
        self
    }
    /// Sets the *android id* query property to the given value.
    ///
    /// 
    /// device android_id
    pub fn android_id(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, NC, A> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PromoofferDismisCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferDismisCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PromoofferDismisCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Returns a list of promo offers available to the user
///
/// A builder for the *get* method supported by a *promooffer* resource.
/// It is not used directly, but through a `PromoofferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().get()
///              .serial("Lorem")
///              .product("Lorem")
///              .model("diam")
///              .manufacturer("ut")
///              .device("ut")
///              .android_id("amet.")
///              .doit();
/// # }
/// ```
pub struct PromoofferGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _serial: Option<String>,
    _product: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PromoofferGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PromoofferGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Offers)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.promooffer.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        if let Some(value) = self._serial {
            params.push(("serial", value.to_string()));
        }
        if let Some(value) = self._product {
            params.push(("product", value.to_string()));
        }
        if let Some(value) = self._model {
            params.push(("model", value.to_string()));
        }
        if let Some(value) = self._manufacturer {
            params.push(("manufacturer", value.to_string()));
        }
        if let Some(value) = self._device {
            params.push(("device", value.to_string()));
        }
        if let Some(value) = self._android_id {
            params.push(("androidId", value.to_string()));
        }
        for &field in ["alt", "serial", "product", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/promooffer/get".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *serial* query property to the given value.
    ///
    /// 
    /// device serial
    pub fn serial(mut self, new_value: &str) -> PromoofferGetCall<'a, C, NC, A> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// Sets the *product* query property to the given value.
    ///
    /// 
    /// device product
    pub fn product(mut self, new_value: &str) -> PromoofferGetCall<'a, C, NC, A> {
        self._product = Some(new_value.to_string());
        self
    }
    /// Sets the *model* query property to the given value.
    ///
    /// 
    /// device model
    pub fn model(mut self, new_value: &str) -> PromoofferGetCall<'a, C, NC, A> {
        self._model = Some(new_value.to_string());
        self
    }
    /// Sets the *manufacturer* query property to the given value.
    ///
    /// 
    /// device manufacturer
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferGetCall<'a, C, NC, A> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// Sets the *device* query property to the given value.
    ///
    /// 
    /// device device
    pub fn device(mut self, new_value: &str) -> PromoofferGetCall<'a, C, NC, A> {
        self._device = Some(new_value.to_string());
        self
    }
    /// Sets the *android id* query property to the given value.
    ///
    /// 
    /// device android_id
    pub fn android_id(mut self, new_value: &str) -> PromoofferGetCall<'a, C, NC, A> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PromoofferGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PromoofferGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List categories for onboarding experience.
///
/// A builder for the *listCategories* method supported by a *onboarding* resource.
/// It is not used directly, but through a `OnboardingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.onboarding().list_categories()
///              .locale("ipsum")
///              .doit();
/// # }
/// ```
pub struct OnboardingListCategoryCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for OnboardingListCategoryCall<'a, C, NC, A> {}

impl<'a, C, NC, A> OnboardingListCategoryCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Category)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.onboarding.listCategories", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/onboarding/listCategories".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset.
    pub fn locale(mut self, new_value: &str) -> OnboardingListCategoryCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OnboardingListCategoryCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OnboardingListCategoryCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> OnboardingListCategoryCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List available volumes under categories for onboarding experience.
///
/// A builder for the *listCategoryVolumes* method supported by a *onboarding* resource.
/// It is not used directly, but through a `OnboardingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.onboarding().list_category_volumes()
///              .page_token("ut")
///              .page_size(98)
///              .locale("sea")
///              .add_category_id("ut")
///              .doit();
/// # }
/// ```
pub struct OnboardingListCategoryVolumeCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _page_token: Option<String>,
    _page_size: Option<u32>,
    _locale: Option<String>,
    _category_id: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for OnboardingListCategoryVolumeCall<'a, C, NC, A> {}

impl<'a, C, NC, A> OnboardingListCategoryVolumeCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volume2)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.onboarding.listCategoryVolumes", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if self._category_id.len() > 0 {
            let mut s = String::new();
            for f in self._category_id.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("categoryId", s));
        }
        for &field in ["alt", "pageToken", "pageSize", "locale", "categoryId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/onboarding/listCategoryVolumes".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The value of the nextToken from the previous page.
    pub fn page_token(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *page size* query property to the given value.
    ///
    /// 
    /// Number of maximum results per page to be included in the response.
    pub fn page_size(mut self, new_value: u32) -> OnboardingListCategoryVolumeCall<'a, C, NC, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset.
    pub fn locale(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *category id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// List of category ids requested.
    pub fn add_category_id(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, C, NC, A> {
        self._category_id.push(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OnboardingListCategoryVolumeCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OnboardingListCategoryVolumeCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> OnboardingListCategoryVolumeCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Request concurrent and download access restrictions.
///
/// A builder for the *requestAccess* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().request_access("source", "volumeId", "nonce", "cpksver")
///              .locale("et")
///              .license_types("et")
///              .doit();
/// # }
/// ```
pub struct MyconfigRequestAccesCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _source: String,
    _volume_id: String,
    _nonce: String,
    _cpksver: String,
    _locale: Option<String>,
    _license_types: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MyconfigRequestAccesCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MyconfigRequestAccesCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RequestAccess)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.requestAccess", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("source", self._source.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("nonce", self._nonce.to_string()));
        params.push(("cpksver", self._cpksver.to_string()));
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._license_types {
            params.push(("licenseTypes", value.to_string()));
        }
        for &field in ["alt", "source", "volumeId", "nonce", "cpksver", "locale", "licenseTypes"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/myconfig/requestAccess".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *source* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        self._source = new_value.to_string();
        self
    }
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume to request concurrent/download restrictions for.
    pub fn volume_id(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *nonce* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The client nonce value.
    pub fn nonce(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        self._nonce = new_value.to_string();
        self
    }
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The device/version ID from which to request the restrictions.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    pub fn locale(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *license types* query property to the given value.
    ///
    /// 
    /// The type of access license to request. If not specified, the default is BOTH.
    pub fn license_types(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        self._license_types = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigRequestAccesCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigRequestAccesCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MyconfigRequestAccesCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Release downloaded content access restriction.
///
/// A builder for the *releaseDownloadAccess* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().release_download_access("volumeIds", "cpksver")
///              .source("sed")
///              .locale("et")
///              .doit();
/// # }
/// ```
pub struct MyconfigReleaseDownloadAccesCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_ids: Vec<String>,
    _cpksver: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MyconfigReleaseDownloadAccesCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DownloadAccesses)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.releaseDownloadAccess", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._volume_ids.len() > 0 {
            let mut s = String::new();
            for f in self._volume_ids.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("volumeIds", s));
        }
        params.push(("cpksver", self._cpksver.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "volumeIds", "cpksver", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/myconfig/releaseDownloadAccess".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Append the given value to the *volume ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The volume(s) to release restrictions for.
    pub fn add_volume_ids(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> {
        self._volume_ids.push(new_value.to_string());
        self
    }
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The device/version ID from which to release the restriction.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    pub fn locale(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MyconfigReleaseDownloadAccesCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Request downloaded content access for specified volumes on the My eBooks shelf.
///
/// A builder for the *syncVolumeLicenses* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().sync_volume_licenses("source", "nonce", "cpksver")
///              .add_volume_ids("vero")
///              .show_preorders(false)
///              .locale("takimata")
///              .add_features("et")
///              .doit();
/// # }
/// ```
pub struct MyconfigSyncVolumeLicenseCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _source: String,
    _nonce: String,
    _cpksver: String,
    _volume_ids: Vec<String>,
    _show_preorders: Option<bool>,
    _locale: Option<String>,
    _features: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.syncVolumeLicenses", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("source", self._source.to_string()));
        params.push(("nonce", self._nonce.to_string()));
        params.push(("cpksver", self._cpksver.to_string()));
        if self._volume_ids.len() > 0 {
            let mut s = String::new();
            for f in self._volume_ids.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("volumeIds", s));
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if self._features.len() > 0 {
            let mut s = String::new();
            for f in self._features.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("features", s));
        }
        for &field in ["alt", "source", "nonce", "cpksver", "volumeIds", "showPreorders", "locale", "features"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/myconfig/syncVolumeLicenses".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *source* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._source = new_value.to_string();
        self
    }
    /// Sets the *nonce* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The client nonce value.
    pub fn nonce(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._nonce = new_value.to_string();
        self
    }
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The device/version ID from which to release the restriction.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// Append the given value to the *volume ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// The volume(s) to request download restrictions for.
    pub fn add_volume_ids(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._volume_ids.push(new_value.to_string());
        self
    }
    /// Sets the *show preorders* query property to the given value.
    ///
    /// 
    /// Set to true to show pre-ordered books. Defaults to false.
    pub fn show_preorders(mut self, new_value: bool) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    pub fn locale(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *features* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// List of features supported by the client, i.e., 'RENTALS'
    pub fn add_features(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._features.push(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MyconfigSyncVolumeLicenseCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the current settings for the user.
///
/// A builder for the *getUserSettings* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().get_user_settings()
///              .doit();
/// # }
/// ```
pub struct MyconfigGetUserSettingCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MyconfigGetUserSettingCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MyconfigGetUserSettingCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Usersettings)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.getUserSettings", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/myconfig/getUserSettings".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigGetUserSettingCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigGetUserSettingCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MyconfigGetUserSettingCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Sets the settings for the user. Unspecified sub-objects will retain the existing value.
///
/// A builder for the *updateUserSettings* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::Usersettings;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Usersettings = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().update_user_settings(&req)
///              .doit();
/// # }
/// ```
pub struct MyconfigUpdateUserSettingCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _request: Usersettings,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MyconfigUpdateUserSettingCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MyconfigUpdateUserSettingCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Usersettings)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.updateUserSettings", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/myconfig/updateUserSettings".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Usersettings) -> MyconfigUpdateUserSettingCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigUpdateUserSettingCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigUpdateUserSettingCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MyconfigUpdateUserSettingCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Clears all volumes from a bookshelf.
///
/// A builder for the *bookshelves.clearVolumes* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_clear_volumes("shelf")
///              .source("et")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveClearVolumeCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryBookshelveClearVolumeCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryBookshelveClearVolumeCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.clearVolumes", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/mylibrary/bookshelves/{shelf}/clearVolumes".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of bookshelf from which to remove a volume.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveClearVolumeCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveClearVolumeCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveClearVolumeCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveClearVolumeCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryBookshelveClearVolumeCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Moves a volume within a bookshelf.
///
/// A builder for the *bookshelves.moveVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_move_volume("shelf", "volumeId", -38)
///              .source("rebum.")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveMoveVolumeCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _shelf: String,
    _volume_id: String,
    _volume_position: i32,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.moveVolume", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("shelf", self._shelf.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("volumePosition", self._volume_position.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["shelf", "volumeId", "volumePosition", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/mylibrary/bookshelves/{shelf}/moveVolume".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of bookshelf with the volume.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of volume to move.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *volume position* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)
    pub fn volume_position(mut self, new_value: i32) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> {
        self._volume_position = new_value;
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryBookshelveMoveVolumeCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets volume information for volumes on a bookshelf.
///
/// A builder for the *bookshelves.volumes.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_volumes_list("shelf")
///              .start_index(19)
///              .source("elitr")
///              .show_preorders(true)
///              .q("sea")
///              .projection("elitr")
///              .max_results(24)
///              .country("sea")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveVolumeListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _shelf: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _q: Option<String>,
    _projection: Option<String>,
    _max_results: Option<u32>,
    _country: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryBookshelveVolumeListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryBookshelveVolumeListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.volumes.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        for &field in ["alt", "shelf", "startIndex", "source", "showPreorders", "q", "projection", "maxResults", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/bookshelves/{shelf}/volumes".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The bookshelf ID or name retrieve volumes for.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *start index* query property to the given value.
    ///
    /// 
    /// Index of the first element to return (starts at 0)
    pub fn start_index(mut self, new_value: u32) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._start_index = Some(new_value);
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *show preorders* query property to the given value.
    ///
    /// 
    /// Set to true to show pre-ordered books. Defaults to false.
    pub fn show_preorders(mut self, new_value: bool) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Sets the *q* query property to the given value.
    ///
    /// 
    /// Full-text search query string in this bookshelf.
    pub fn q(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Restrict information returned to a set of selected fields.
    pub fn projection(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return
    pub fn max_results(mut self, new_value: u32) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *country* query property to the given value.
    ///
    /// 
    /// ISO-3166-1 code to override the IP-based location.
    pub fn country(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryBookshelveVolumeListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the summary of specified layers.
///
/// A builder for the *annotations.summary* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_summary("layerIds", "volumeId")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationSummaryCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _layer_ids: Vec<String>,
    _volume_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryAnnotationSummaryCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryAnnotationSummaryCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AnnotationsSummary)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.summary", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if self._layer_ids.len() > 0 {
            let mut s = String::new();
            for f in self._layer_ids.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("layerIds", s));
        }
        params.push(("volumeId", self._volume_id.to_string()));
        for &field in ["alt", "layerIds", "volumeId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/annotations/summary".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Append the given value to the *layer ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Array of layer IDs to get the summary for.
    pub fn add_layer_ids(mut self, new_value: &str) -> MylibraryAnnotationSummaryCall<'a, C, NC, A> {
        self._layer_ids.push(new_value.to_string());
        self
    }
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Volume id to get the summary for.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryAnnotationSummaryCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationSummaryCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationSummaryCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryAnnotationSummaryCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Deletes an annotation.
///
/// A builder for the *annotations.delete* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_delete("annotationId")
///              .source("dolores")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _annotation_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryAnnotationDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryAnnotationDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("annotationId", self._annotation_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["annotationId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/mylibrary/annotations/{annotationId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{annotationId}", "annotationId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["annotationId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID for the annotation to delete.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationDeleteCall<'a, C, NC, A> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationDeleteCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationDeleteCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryAnnotationDeleteCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Adds a volume to a bookshelf.
///
/// A builder for the *bookshelves.addVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_add_volume("shelf", "volumeId")
///              .source("aliquyam")
///              .reason("elitr")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveAddVolumeCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _shelf: String,
    _volume_id: String,
    _source: Option<String>,
    _reason: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryBookshelveAddVolumeCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.addVolume", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("shelf", self._shelf.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._reason {
            params.push(("reason", value.to_string()));
        }
        for &field in ["shelf", "volumeId", "source", "reason"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/mylibrary/bookshelves/{shelf}/addVolume".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of bookshelf to which to add a volume.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of volume to add.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *reason* query property to the given value.
    ///
    /// 
    /// The reason for which the book is added to the library.
    pub fn reason(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> {
        self._reason = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryBookshelveAddVolumeCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Inserts a new annotation.
///
/// A builder for the *annotations.insert* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::Annotation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Annotation = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_insert(&req)
///              .source("ea")
///              .show_only_summary_in_response(true)
///              .country("Stet")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _request: Annotation,
    _source: Option<String>,
    _show_only_summary_in_response: Option<bool>,
    _country: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryAnnotationInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryAnnotationInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_only_summary_in_response {
            params.push(("showOnlySummaryInResponse", value.to_string()));
        }
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        for &field in ["alt", "source", "showOnlySummaryInResponse", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/annotations".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Annotation) -> MylibraryAnnotationInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *show only summary in response* query property to the given value.
    ///
    /// 
    /// Requests that only the summary of the specified layer be provided in the response.
    pub fn show_only_summary_in_response(mut self, new_value: bool) -> MylibraryAnnotationInsertCall<'a, C, NC, A> {
        self._show_only_summary_in_response = Some(new_value);
        self
    }
    /// Sets the *country* query property to the given value.
    ///
    /// 
    /// ISO-3166-1 code to override the IP-based location.
    pub fn country(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, C, NC, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationInsertCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryAnnotationInsertCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Removes a volume from a bookshelf.
///
/// A builder for the *bookshelves.removeVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_remove_volume("shelf", "volumeId")
///              .source("sanctus")
///              .reason("dolore")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _shelf: String,
    _volume_id: String,
    _source: Option<String>,
    _reason: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.removeVolume", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("shelf", self._shelf.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._reason {
            params.push(("reason", value.to_string()));
        }
        for &field in ["shelf", "volumeId", "source", "reason"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/mylibrary/bookshelves/{shelf}/removeVolume".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of bookshelf from which to remove a volume.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of volume to remove.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *reason* query property to the given value.
    ///
    /// 
    /// The reason for which the book is removed from the library.
    pub fn reason(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> {
        self._reason = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryBookshelveRemoveVolumeCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves a list of annotations, possibly filtered.
///
/// A builder for the *annotations.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_list()
///              .volume_id("Lorem")
///              .updated_min("consetetur")
///              .updated_max("consetetur")
///              .source("eirmod")
///              .show_deleted(true)
///              .page_token("gubergren")
///              .max_results(28)
///              .add_layer_ids("sadipscing")
///              .layer_id("accusam")
///              .content_version("magna")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: Option<String>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _source: Option<String>,
    _show_deleted: Option<bool>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _layer_ids: Vec<String>,
    _layer_id: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryAnnotationListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryAnnotationListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotations)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        if let Some(value) = self._volume_id {
            params.push(("volumeId", value.to_string()));
        }
        if let Some(value) = self._updated_min {
            params.push(("updatedMin", value.to_string()));
        }
        if let Some(value) = self._updated_max {
            params.push(("updatedMax", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_deleted {
            params.push(("showDeleted", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if self._layer_ids.len() > 0 {
            let mut s = String::new();
            for f in self._layer_ids.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("layerIds", s));
        }
        if let Some(value) = self._layer_id {
            params.push(("layerId", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "updatedMin", "updatedMax", "source", "showDeleted", "pageToken", "maxResults", "layerIds", "layerId", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/annotations".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* query property to the given value.
    ///
    /// 
    /// The volume to restrict annotations to.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// Sets the *updated min* query property to the given value.
    ///
    /// 
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    pub fn updated_min(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// Sets the *updated max* query property to the given value.
    ///
    /// 
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    pub fn updated_max(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *show deleted* query property to the given value.
    ///
    /// 
    /// Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false.
    pub fn show_deleted(mut self, new_value: bool) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._show_deleted = Some(new_value);
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The value of the nextToken from the previous page.
    pub fn page_token(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return
    pub fn max_results(mut self, new_value: u32) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Append the given value to the *layer ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// The layer ID(s) to limit annotation by.
    pub fn add_layer_ids(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._layer_ids.push(new_value.to_string());
        self
    }
    /// Sets the *layer id* query property to the given value.
    ///
    /// 
    /// The layer ID to limit annotation by.
    pub fn layer_id(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._layer_id = Some(new_value.to_string());
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// 
    /// The content version for the requested volume.
    pub fn content_version(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryAnnotationListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Updates an existing annotation.
///
/// A builder for the *annotations.update* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::Annotation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Annotation = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_update(&req, "annotationId")
///              .source("rebum.")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _request: Annotation,
    _annotation_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryAnnotationUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryAnnotationUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("annotationId", self._annotation_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "annotationId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/annotations/{annotationId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{annotationId}", "annotationId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["annotationId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Annotation) -> MylibraryAnnotationUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID for the annotation to update.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationUpdateCall<'a, C, NC, A> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationUpdateCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationUpdateCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryAnnotationUpdateCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Sets my reading position information for a volume.
///
/// A builder for the *readingpositions.setPosition* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().readingpositions_set_position("volumeId", "timestamp", "position")
///              .source("dolores")
///              .device_cookie("vero")
///              .content_version("consetetur")
///              .action("vero")
///              .doit();
/// # }
/// ```
pub struct MylibraryReadingpositionSetPositionCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _timestamp: String,
    _position: String,
    _source: Option<String>,
    _device_cookie: Option<String>,
    _content_version: Option<String>,
    _action: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.readingpositions.setPosition", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("timestamp", self._timestamp.to_string()));
        params.push(("position", self._position.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._device_cookie {
            params.push(("deviceCookie", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        if let Some(value) = self._action {
            params.push(("action", value.to_string()));
        }
        for &field in ["volumeId", "timestamp", "position", "source", "deviceCookie", "contentVersion", "action"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/mylibrary/readingpositions/{volumeId}/setPosition".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["volumeId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of volume for which to update the reading position.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *timestamp* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// RFC 3339 UTC format timestamp associated with this reading position.
    pub fn timestamp(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._timestamp = new_value.to_string();
        self
    }
    /// Sets the *position* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Position string for the new volume reading position.
    pub fn position(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._position = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *device cookie* query property to the given value.
    ///
    /// 
    /// Random persistent device cookie optional on set position.
    pub fn device_cookie(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._device_cookie = Some(new_value.to_string());
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// 
    /// Volume content version for which this reading position applies.
    pub fn content_version(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// Sets the *action* query property to the given value.
    ///
    /// 
    /// Action that caused this reading position to be set.
    pub fn action(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._action = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryReadingpositionSetPositionCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves metadata for a specific bookshelf belonging to the authenticated user.
///
/// A builder for the *bookshelves.get* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_get("shelf")
///              .source("eos")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryBookshelveGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryBookshelveGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelf)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/bookshelves/{shelf}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["shelf"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of bookshelf to retrieve.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveGetCall<'a, C, NC, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveGetCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryBookshelveGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves a list of bookshelves belonging to the authenticated user.
///
/// A builder for the *bookshelves.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_list()
///              .source("justo")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryBookshelveListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryBookshelveListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelves)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/bookshelves".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveListCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryBookshelveListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves my reading position information for a volume.
///
/// A builder for the *readingpositions.get* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().readingpositions_get("volumeId")
///              .source("gubergren")
///              .content_version("dolore")
///              .doit();
/// # }
/// ```
pub struct MylibraryReadingpositionGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _source: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MylibraryReadingpositionGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MylibraryReadingpositionGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ReadingPosition)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.readingpositions.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "source", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/mylibrary/readingpositions/{volumeId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["volumeId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// ID of volume for which to retrieve a reading position.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *source* query property to the given value.
    ///
    /// 
    /// String to identify the originator of this request.
    pub fn source(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, C, NC, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Sets the *content version* query property to the given value.
    ///
    /// 
    /// Volume content version for which this reading position is requested.
    pub fn content_version(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, C, NC, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryReadingpositionGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryReadingpositionGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MylibraryReadingpositionGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *addBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a `CloudloadingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().add_book()
///              .upload_client_token("amet.")
///              .name("dolore")
///              .mime_type("magna")
///              .drive_document_id("elitr")
///              .doit();
/// # }
/// ```
pub struct CloudloadingAddBookCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _upload_client_token: Option<String>,
    _name: Option<String>,
    _mime_type: Option<String>,
    _drive_document_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for CloudloadingAddBookCall<'a, C, NC, A> {}

impl<'a, C, NC, A> CloudloadingAddBookCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BooksCloudloadingResource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.cloudloading.addBook", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if let Some(value) = self._upload_client_token {
            params.push(("upload_client_token", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._mime_type {
            params.push(("mime_type", value.to_string()));
        }
        if let Some(value) = self._drive_document_id {
            params.push(("drive_document_id", value.to_string()));
        }
        for &field in ["alt", "upload_client_token", "name", "mime_type", "drive_document_id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/cloudloading/addBook".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *upload_client_token* query property to the given value.
    ///
    /// 
    pub fn upload_client_token(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, NC, A> {
        self._upload_client_token = Some(new_value.to_string());
        self
    }
    /// Sets the *name* query property to the given value.
    ///
    /// 
    /// The document name. It can be set only if the drive_document_id is set.
    pub fn name(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, NC, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Sets the *mime_type* query property to the given value.
    ///
    /// 
    /// The document MIME type. It can be set only if the drive_document_id is set.
    pub fn mime_type(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, NC, A> {
        self._mime_type = Some(new_value.to_string());
        self
    }
    /// Sets the *drive_document_id* query property to the given value.
    ///
    /// 
    /// A drive document id. The upload_client_token must not be set.
    pub fn drive_document_id(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, NC, A> {
        self._drive_document_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CloudloadingAddBookCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingAddBookCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> CloudloadingAddBookCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *updateBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a `CloudloadingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::BooksCloudloadingResource;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: BooksCloudloadingResource = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().update_book(&req)
///              .doit();
/// # }
/// ```
pub struct CloudloadingUpdateBookCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _request: BooksCloudloadingResource,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for CloudloadingUpdateBookCall<'a, C, NC, A> {}

impl<'a, C, NC, A> CloudloadingUpdateBookCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BooksCloudloadingResource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.cloudloading.updateBook", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/books/v1/cloudloading/updateBook".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &BooksCloudloadingResource) -> CloudloadingUpdateBookCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CloudloadingUpdateBookCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingUpdateBookCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> CloudloadingUpdateBookCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Remove the book and its contents
///
/// A builder for the *deleteBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a `CloudloadingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().delete_book("volumeId")
///              .doit();
/// # }
/// ```
pub struct CloudloadingDeleteBookCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Books<C, NC, A>,
    _volume_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for CloudloadingDeleteBookCall<'a, C, NC, A> {}

impl<'a, C, NC, A> CloudloadingDeleteBookCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.cloudloading.deleteBook", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        params.push(("volumeId", self._volume_id.to_string()));
        for &field in ["volumeId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/books/v1/cloudloading/deleteBook".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id of the book to be removed.
    pub fn volume_id(mut self, new_value: &str) -> CloudloadingDeleteBookCall<'a, C, NC, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CloudloadingDeleteBookCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingDeleteBookCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> CloudloadingDeleteBookCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}

