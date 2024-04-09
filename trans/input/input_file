#include <stdio.h>
#include <stdlib.h>

#define Error( Str )        FatalError( Str )
#define FatalError( Str )   fprintf( stderr, "%s\n", Str ), exit( 1 )

typedef int ElementType;

struct TreeNode;
typedef struct TreeNode *Position;
typedef struct TreeNode *SearchTree;

SearchTree MakeEmpty( SearchTree T );
Position Find( ElementType X, SearchTree T );
Position FindMin( SearchTree T );
Position FindMax( SearchTree T );
SearchTree Insert( ElementType X, SearchTree T );
SearchTree Delete( ElementType X, SearchTree T );
ElementType Retrieve( Position P );

struct TreeNode
{
        ElementType Element;
        SearchTree  Left;
        SearchTree  Right;
};

SearchTree
MakeEmpty( SearchTree T )
{
        if( T != NULL )
        {
        MakeEmpty( T->Left );
        MakeEmpty( T->Right );
        free( T );
        }
        return NULL;
}

Position
Find( ElementType X, SearchTree T )
{
        if( T == NULL )
        return NULL;
        if( X < T->Element )
        return Find( X, T->Left );
        else
        if( X > T->Element )
        return Find( X, T->Right );
        else
        return T;
}

Position
FindMin( SearchTree T )
{
        if( T == NULL )
        return NULL;
        else
        if( T->Left == NULL )
        return T;
        else
        return FindMin( T->Left );
}

Position
FindMax( SearchTree T )
{
        if( T != NULL )
        while( T->Right != NULL )
                T = T->Right;

        return T;
}

SearchTree
Insert( ElementType X, SearchTree T )
{
    if( T == NULL )
    {
        T = malloc( sizeof( struct TreeNode ) );
        if( T == NULL )
             FatalError( "Out of space!!!" );
        else
        {
              T->Element = X;
              T->Left = T->Right = NULL;
        }
    }
    else
      if( X < T->Element )
          T->Left = Insert( X, T->Left );
      else
           if( X > T->Element )
                T->Right = Insert( X, T->Right );
        

    return T;
}


SearchTree
Delete( ElementType X, SearchTree T )
{
        Position TmpCell;

        if( T == NULL )
        Error( "Element not found" );
        else
        if( X < T->Element )  
        T->Left = Delete( X, T->Left );
        else
        if( X > T->Element )  
        T->Right = Delete( X, T->Right );
        else 
        if( T->Left && T->Right )  
        {
        
        TmpCell = FindMin( T->Right );
        T->Element = TmpCell->Element;
        T->Right = Delete( T->Element, T->Right );
        }
        else  
        {
        TmpCell = T;
        if( T->Left == NULL ) 
                T = T->Right;
        else if( T->Right == NULL )
                T = T->Left;
        free( TmpCell );
        }

        return T;
}

ElementType
Retrieve( Position P )
{
        return P->Element;
}

int main( )
{
    SearchTree T;
    Position P;
    int i;
    int j = 0;

    T = MakeEmpty( NULL );
    for( i = 0; i < 50; i++, j = ( j + 7 ) % 50 )
        T = Insert( j, T );
    for( i = 0; i < 50; i++ )
        if( ( P = Find( i, T ) ) == NULL || Retrieve( P ) != i )
            printf( "Error at %d\n", i );

    for( i = 0; i < 50; i += 2 )
        T = Delete( i, T );

    for( i = 1; i < 50; i += 2 )
        if( ( P = Find( i, T ) ) == NULL || Retrieve( P ) != i )
            printf( "Error at %d\n", i );
    for( i = 0; i < 50; i += 2 )
        if( ( P = Find( i, T ) ) != NULL )
            printf( "Error at %d\n", i );

    printf( "Min is %d, Max is %d\n", Retrieve( FindMin( T ) ),
               Retrieve( FindMax( T ) ) );

    return 0;
}