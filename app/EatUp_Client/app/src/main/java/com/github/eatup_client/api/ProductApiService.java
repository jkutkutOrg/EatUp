package com.github.eatup_client.api;

import android.content.Context;
import android.util.Log;

import androidx.lifecycle.LiveData;
import androidx.lifecycle.MutableLiveData;

import com.github.eatup_client.model.Allergy;
import com.github.eatup_client.model.Category;
import com.github.eatup_client.model.OrderProduct;
import com.github.eatup_client.model.Product;
import com.github.eatup_client.model.MVCManager;
import com.github.eatup_client.model.Session;
import com.github.eatup_client.model.SessionId;
import com.google.gson.Gson;
import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

import java.io.IOException;
import java.util.List;

import okhttp3.Cache;
import okhttp3.OkHttpClient;
import okhttp3.ResponseBody;
import retrofit2.Call;
import retrofit2.Callback;
import retrofit2.Response;
import retrofit2.Retrofit;
import retrofit2.converter.gson.GsonConverterFactory;
import retrofit2.http.Body;
import retrofit2.http.GET;
import retrofit2.http.Headers;
import retrofit2.http.POST;
import retrofit2.http.Path;
import retrofit2.http.Query;

/**
 * Service class for making API calls related to products.
 */
public class ProductApiService {

    private static final String BASE_URL = "http://159.69.216.101/api/v1/";
    private static final String ENDPOINT_PRODUCTS = "products";
    private static final String ENDPOINT_SESSIONS = "sessions";
    private static final String ENDPOINT_SESSIONS_ID = "session_id";

    private static String userUUID = "1234567890";
    private static String TAG = "ProductApiService";

    private static ProductApiService sInstance;
    private final ProductService mProductService;
    private final SessionService mSessionService;
    private final SessionIdService mSessionIdService;

    private Context mContext;

    public ProductApiService(Context context) {
        mContext = context;
        OkHttpClient client = new OkHttpClient.Builder()
                .cache(new Cache(mContext.getCacheDir(), 10 * 1024 * 1024)) // 10 MB cache
                .build();
        Retrofit retrofit = new Retrofit.Builder()
                .baseUrl(BASE_URL)
                .client(client)
                .addConverterFactory(GsonConverterFactory.create())
                .build();

        mProductService = retrofit.create(ProductService.class);
        mSessionService = retrofit.create(SessionService.class);
        mSessionIdService = retrofit.create(SessionIdService.class);
    }

    /**
     * Returns the singleton instance of ProductApiService.
     *
     * @param context the application context
     * @return the ProductApiService instance
     */
    public static synchronized ProductApiService getInstance(Context context) {
        if (sInstance == null) {
            sInstance = new ProductApiService(context.getApplicationContext());
        }
        return sInstance;
    }

    /**
     * Retrieves a list of products by category from the API.
     *
     * @param category the category to filter the products
     * @return LiveData containing the list of products
     */
    public LiveData<List<Product>> getProductsByCategory(String category) {
        MutableLiveData<List<Product>> data = new MutableLiveData<>();
        Call<List<Product>> call = mProductService.getProductsByCategory(category);
        Log.i(TAG, "getProductsByCategory: " + call.request().url());
        call.enqueue(new Callback<List<Product>>() {
            @Override
            public void onResponse(Call<List<Product>> call, Response<List<Product>> response) {
                if (response.isSuccessful()) {
                    data.setValue(response.body());
                    Log.i(TAG, "getProductsByCategory success: " + new Gson().toJson(response.body()));
                } else {
                    Log.e(TAG, "getProductsByCategory error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<List<Product>> call, Throwable t) {
                Log.e(TAG, "getProductsByCategory failure: " + t.getMessage());
            }
        });
        return data;
    }

    /**
     * Checks if a QR code is valid and returns the result as LiveData.
     *
     * @param qrCode the QR code to validate
     * @return LiveData indicating if the QR code is valid
     */
    public LiveData<Boolean> isQRValid(String qrCode) {
        MutableLiveData<Boolean> data = new MutableLiveData<>();

        Session session = MVCManager.getInstance().getSession(qrCode);
        if (session != null) {
            userUUID = session.getId();
            data.setValue(true);
        } else {
            data.setValue(false);
        }

        return data;
    }

    /**
     * Retrieves a list of sessions from the API.
     *
     * @return LiveData containing the list of sessions
     */
    public LiveData<List<Session>> loadSessions() {
        MutableLiveData<List<Session>> data = new MutableLiveData<>();
        Call<List<Session>> call = mSessionService.getSessions();
        call.enqueue(new Callback<List<Session>>() {
            @Override
            public void onResponse(Call<List<Session>> call, Response<List<Session>> response) {
                if (response.isSuccessful()) {
                    List<Session> sessions = response.body();
                    for (Session session : sessions) {
                        MVCManager.getInstance().addSession(session);
                    }
                    data.setValue(sessions);
                    Log.i(TAG, "loadSessions success: " + new Gson().toJson(response.body()));
                } else {
                    Log.e(TAG, "loadSessions error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<List<Session>> call, Throwable t) {
                Log.e(TAG, "loadSessions failure: " + t.getMessage());
            }
        });

        return data;
    }

    /**
     * Retrieves a valid session ID from the API.
     *
     * @param sessionId the session ID to validate
     * @return LiveData containing the session ID if valid, null otherwise
     */
    public LiveData<SessionId> getValidSession(String sessionId) {
        MutableLiveData<SessionId> data = new MutableLiveData<>();
        Call<SessionId> call = mSessionIdService.getSessionId(sessionId);
        Log.i(TAG, "getValidSession: " + call.request().url());
        call.enqueue(new Callback<SessionId>() {
            @Override
            public void onResponse(Call<SessionId> call, Response<SessionId> response) {
                if (response.isSuccessful()) {
                    SessionId sessionId = response.body();
                    userUUID = sessionId.getId();
                    Log.i(TAG, "getValidSession success: " + new Gson().toJson(response.body()));
                    data.setValue(sessionId);
                } else {
                    data.setValue(null);
                    Log.e(TAG, "getValidSession error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<SessionId> call, Throwable t) {
                data.setValue(null);
                Log.e(TAG, "getValidSession failure: " + t.getMessage());
            }
        });

        return data;
    }

    /**
     * Submits an order to the API.
     *
     * @param orderProducts the list of ordered products
     */
    public void submitOrder(List<OrderProduct> orderProducts) {
        JsonObject requestBody = new JsonObject();

        JsonArray productsArray = new JsonArray();

        for (OrderProduct orderProduct : orderProducts) {
            JsonObject productObject = new JsonObject();

            productObject.addProperty("quantity", orderProduct.getQuantity());

            JsonObject product = new JsonObject();

            product.addProperty("id", orderProduct.getProduct().getId());
            product.addProperty("name", orderProduct.getProduct().getName());
            product.addProperty("description", orderProduct.getProduct().getDescription());
            product.addProperty("img_id", orderProduct.getProduct().getImg_id());
            product.addProperty("price", orderProduct.getProduct().getPrice());

            JsonArray allergiesArray = new JsonArray();

            for (Allergy allergy : orderProduct.getProduct().getAllergies()) {
                JsonObject allergyObject = new JsonObject();
                allergyObject.addProperty("id", allergy.getId());
                allergyObject.addProperty("name", allergy.getName());
                allergyObject.addProperty("img_id", allergy.getImg_id());
                allergiesArray.add(allergyObject);
            }

            product.add("allergies", allergiesArray);

            JsonArray categoriesArray = new JsonArray();

            for (Category category : orderProduct.getProduct().getCategories()) {
                JsonObject categoryObject = new JsonObject();
                categoryObject.addProperty("id", category.getId());
                categoryObject.addProperty("name", category.getName());
                categoriesArray.add(categoryObject);
            }

            product.add("categories", categoriesArray);

            productObject.add("product", product);

            productsArray.add(productObject);
        }

        requestBody.add("products", productsArray);

        String jsonBody = requestBody.toString();
        Log.i(TAG, "JSON body: " + jsonBody);

        Call<ResponseBody> call = mProductService.submitOrder(userUUID, requestBody);
        Log.i(TAG, "submitOrder: " + call.request().url());
        Log.i(TAG, "submitOrder 1: " + call.request().toString());
        call.enqueue(new Callback<ResponseBody>() {
            @Override
            public void onResponse(Call<ResponseBody> call, Response<ResponseBody> response) {
                if (response.isSuccessful()) {
                    Log.i(TAG, "submitOrder success: " + response.body().toString());
                } else {
                    Log.e(TAG, "submitOrder error: " + response.code());
                    try {
                        String errorBody = response.errorBody().string();
                        Log.e(TAG, "Error body: " + errorBody);
                    } catch (IOException e) {
                        e.printStackTrace();
                    }
                }
            }

            @Override
            public void onFailure(Call<ResponseBody> call, Throwable t) {
                Log.e(TAG, "submitOrder failure: " + t.getMessage());
                // Handle failure
            }
        });
    }

    /**
     * Retrofit service interface for product-related API endpoints.
     */
    public interface ProductService {
        @Headers({"device: android"})
        @GET(ENDPOINT_PRODUCTS)
        Call<List<Product>> getProductsByCategory(@Query("category") String category);

        @POST("orders/{userUUID}")
        Call<ResponseBody> submitOrder(
                @Path("userUUID") String userUUID,
                @Body JsonObject orderProducts
        );
    }

    /**
     * Retrofit service interface for session-related API endpoints.
     */
    public interface SessionService {
        @Headers({"device: android"})
        @GET(ENDPOINT_SESSIONS)
        Call<List<Session>> getSessions();
    }

    /**
     * Retrofit service interface for session ID-related API endpoints.
     */
    public interface SessionIdService {
        @Headers({"device: android"})
        @GET(ENDPOINT_SESSIONS_ID + "/{session_id}")
        Call<SessionId> getSessionId(@Path("session_id") String session_id);
    }
}
